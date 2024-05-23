extern "C" {
	// cv::Ptr<cv::dnn::RequantizeLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::RequantizeLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::RequantizeLayer* cv_PtrLcv_dnn_RequantizeLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::RequantizeLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::RequantizeLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::RequantizeLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::RequantizeLayer* cv_PtrLcv_dnn_RequantizeLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::RequantizeLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::RequantizeLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::RequantizeLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::RequantizeLayer>* cv_PtrLcv_dnn_RequantizeLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::RequantizeLayer>();
	}

	// cv::Ptr<cv::dnn::RequantizeLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::RequantizeLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_RequantizeLayerG_delete(cv::Ptr<cv::dnn::RequantizeLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::RequantizeLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::RequantizeLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_RequantizeLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::RequantizeLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::RequantizeLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::RequantizeLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_RequantizeLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::RequantizeLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::RequantizeLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::RequantizeLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::RequantizeLayer"]), _)]),
	cv::Ptr<cv::dnn::RequantizeLayer>* cv_PtrLcv_dnn_RequantizeLayerG_new_const_RequantizeLayer(cv::dnn::RequantizeLayer* val) {
			return new cv::Ptr<cv::dnn::RequantizeLayer>(val);
	}

}

