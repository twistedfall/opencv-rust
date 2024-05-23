extern "C" {
	// cv::Ptr<cv::dnn::ConvolutionLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::ConvolutionLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::ConvolutionLayer* cv_PtrLcv_dnn_ConvolutionLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::ConvolutionLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ConvolutionLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::ConvolutionLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ConvolutionLayer* cv_PtrLcv_dnn_ConvolutionLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::ConvolutionLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ConvolutionLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::ConvolutionLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::ConvolutionLayer>* cv_PtrLcv_dnn_ConvolutionLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::ConvolutionLayer>();
	}

	// cv::Ptr<cv::dnn::ConvolutionLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::ConvolutionLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_ConvolutionLayerG_delete(cv::Ptr<cv::dnn::ConvolutionLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::ConvolutionLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::ConvolutionLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_ConvolutionLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::ConvolutionLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::ConvolutionLayer>::to_PtrOfBaseConvolutionLayer() generated
	// ("cv::Ptr<cv::dnn::ConvolutionLayer>::to_PtrOfBaseConvolutionLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::BaseConvolutionLayer>* cv_PtrLcv_dnn_ConvolutionLayerG_to_PtrOfBaseConvolutionLayer(cv::Ptr<cv::dnn::ConvolutionLayer>* instance) {
			return new cv::Ptr<cv::dnn::BaseConvolutionLayer>(instance->dynamicCast<cv::dnn::BaseConvolutionLayer>());
	}

	// cv::Ptr<cv::dnn::ConvolutionLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::ConvolutionLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_ConvolutionLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::ConvolutionLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::ConvolutionLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::ConvolutionLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::ConvolutionLayer"]), _)]),
	cv::Ptr<cv::dnn::ConvolutionLayer>* cv_PtrLcv_dnn_ConvolutionLayerG_new_const_ConvolutionLayer(cv::dnn::ConvolutionLayer* val) {
			return new cv::Ptr<cv::dnn::ConvolutionLayer>(val);
	}

}

