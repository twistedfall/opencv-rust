extern "C" {
	// cv::Ptr<cv::dnn::DeconvolutionLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::DeconvolutionLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::DeconvolutionLayer* cv_PtrLcv_dnn_DeconvolutionLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::DeconvolutionLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::DeconvolutionLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::DeconvolutionLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::DeconvolutionLayer* cv_PtrLcv_dnn_DeconvolutionLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::DeconvolutionLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::DeconvolutionLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::DeconvolutionLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::DeconvolutionLayer>* cv_PtrLcv_dnn_DeconvolutionLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::DeconvolutionLayer>();
	}

	// cv::Ptr<cv::dnn::DeconvolutionLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::DeconvolutionLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_DeconvolutionLayerG_delete(cv::Ptr<cv::dnn::DeconvolutionLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::DeconvolutionLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::DeconvolutionLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_DeconvolutionLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::DeconvolutionLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::DeconvolutionLayer>::to_PtrOfBaseConvolutionLayer() generated
	// ("cv::Ptr<cv::dnn::DeconvolutionLayer>::to_PtrOfBaseConvolutionLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::BaseConvolutionLayer>* cv_PtrLcv_dnn_DeconvolutionLayerG_to_PtrOfBaseConvolutionLayer(cv::Ptr<cv::dnn::DeconvolutionLayer>* instance) {
			return new cv::Ptr<cv::dnn::BaseConvolutionLayer>(instance->dynamicCast<cv::dnn::BaseConvolutionLayer>());
	}

	// cv::Ptr<cv::dnn::DeconvolutionLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::DeconvolutionLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_DeconvolutionLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::DeconvolutionLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::DeconvolutionLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::DeconvolutionLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::DeconvolutionLayer"]), _)]),
	cv::Ptr<cv::dnn::DeconvolutionLayer>* cv_PtrLcv_dnn_DeconvolutionLayerG_new_const_DeconvolutionLayer(cv::dnn::DeconvolutionLayer* val) {
			return new cv::Ptr<cv::dnn::DeconvolutionLayer>(val);
	}

}

