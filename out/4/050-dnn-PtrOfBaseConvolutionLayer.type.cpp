extern "C" {
	// cv::Ptr<cv::dnn::BaseConvolutionLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::BaseConvolutionLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::BaseConvolutionLayer* cv_PtrLcv_dnn_BaseConvolutionLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::BaseConvolutionLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::BaseConvolutionLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::BaseConvolutionLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::BaseConvolutionLayer* cv_PtrLcv_dnn_BaseConvolutionLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::BaseConvolutionLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::BaseConvolutionLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::BaseConvolutionLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::BaseConvolutionLayer>* cv_PtrLcv_dnn_BaseConvolutionLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::BaseConvolutionLayer>();
	}

	// cv::Ptr<cv::dnn::BaseConvolutionLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::BaseConvolutionLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_BaseConvolutionLayerG_delete(cv::Ptr<cv::dnn::BaseConvolutionLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::BaseConvolutionLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::BaseConvolutionLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_BaseConvolutionLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::BaseConvolutionLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::BaseConvolutionLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::BaseConvolutionLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_BaseConvolutionLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::BaseConvolutionLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::BaseConvolutionLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::BaseConvolutionLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::BaseConvolutionLayer"]), _)]),
	cv::Ptr<cv::dnn::BaseConvolutionLayer>* cv_PtrLcv_dnn_BaseConvolutionLayerG_new_const_BaseConvolutionLayer(cv::dnn::BaseConvolutionLayer* val) {
			return new cv::Ptr<cv::dnn::BaseConvolutionLayer>(val);
	}

}

