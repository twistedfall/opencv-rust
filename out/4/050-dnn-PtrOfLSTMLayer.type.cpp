extern "C" {
	// cv::Ptr<cv::dnn::LSTMLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::LSTMLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::LSTMLayer* cv_PtrLcv_dnn_LSTMLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::LSTMLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::LSTMLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::LSTMLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::LSTMLayer* cv_PtrLcv_dnn_LSTMLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::LSTMLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::LSTMLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::LSTMLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::LSTMLayer>* cv_PtrLcv_dnn_LSTMLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::LSTMLayer>();
	}

	// cv::Ptr<cv::dnn::LSTMLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::LSTMLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_LSTMLayerG_delete(cv::Ptr<cv::dnn::LSTMLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::LSTMLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::LSTMLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_LSTMLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::LSTMLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::LSTMLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::LSTMLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_LSTMLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::LSTMLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

}

