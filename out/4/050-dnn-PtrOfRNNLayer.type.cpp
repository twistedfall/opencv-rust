extern "C" {
	// cv::Ptr<cv::dnn::RNNLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::RNNLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::RNNLayer* cv_PtrLcv_dnn_RNNLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::RNNLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::RNNLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::RNNLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::RNNLayer* cv_PtrLcv_dnn_RNNLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::RNNLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::RNNLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::RNNLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::RNNLayer>* cv_PtrLcv_dnn_RNNLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::RNNLayer>();
	}

	// cv::Ptr<cv::dnn::RNNLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::RNNLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_RNNLayerG_delete(cv::Ptr<cv::dnn::RNNLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::RNNLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::RNNLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_RNNLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::RNNLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::RNNLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::RNNLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_RNNLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::RNNLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

}

