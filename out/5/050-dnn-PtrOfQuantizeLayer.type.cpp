extern "C" {
	// cv::Ptr<cv::dnn::QuantizeLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::QuantizeLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::QuantizeLayer* cv_PtrLcv_dnn_QuantizeLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::QuantizeLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::QuantizeLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::QuantizeLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::QuantizeLayer* cv_PtrLcv_dnn_QuantizeLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::QuantizeLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::QuantizeLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::QuantizeLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::QuantizeLayer>* cv_PtrLcv_dnn_QuantizeLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::QuantizeLayer>();
	}

	// cv::Ptr<cv::dnn::QuantizeLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::QuantizeLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_QuantizeLayerG_delete(cv::Ptr<cv::dnn::QuantizeLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::QuantizeLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::QuantizeLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_QuantizeLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::QuantizeLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::QuantizeLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::QuantizeLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_QuantizeLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::QuantizeLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::QuantizeLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::QuantizeLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::QuantizeLayer"]), _)]),
	cv::Ptr<cv::dnn::QuantizeLayer>* cv_PtrLcv_dnn_QuantizeLayerG_new_const_QuantizeLayer(cv::dnn::QuantizeLayer* val) {
			return new cv::Ptr<cv::dnn::QuantizeLayer>(val);
	}

}

