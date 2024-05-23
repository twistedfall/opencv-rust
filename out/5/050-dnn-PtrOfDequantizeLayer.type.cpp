extern "C" {
	// cv::Ptr<cv::dnn::DequantizeLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::DequantizeLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::DequantizeLayer* cv_PtrLcv_dnn_DequantizeLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::DequantizeLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::DequantizeLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::DequantizeLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::DequantizeLayer* cv_PtrLcv_dnn_DequantizeLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::DequantizeLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::DequantizeLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::DequantizeLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::DequantizeLayer>* cv_PtrLcv_dnn_DequantizeLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::DequantizeLayer>();
	}

	// cv::Ptr<cv::dnn::DequantizeLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::DequantizeLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_DequantizeLayerG_delete(cv::Ptr<cv::dnn::DequantizeLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::DequantizeLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::DequantizeLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_DequantizeLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::DequantizeLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::DequantizeLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::DequantizeLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_DequantizeLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::DequantizeLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::DequantizeLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::DequantizeLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::DequantizeLayer"]), _)]),
	cv::Ptr<cv::dnn::DequantizeLayer>* cv_PtrLcv_dnn_DequantizeLayerG_new_const_DequantizeLayer(cv::dnn::DequantizeLayer* val) {
			return new cv::Ptr<cv::dnn::DequantizeLayer>(val);
	}

}

