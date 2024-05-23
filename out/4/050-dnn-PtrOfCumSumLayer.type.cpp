extern "C" {
	// cv::Ptr<cv::dnn::CumSumLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::CumSumLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::CumSumLayer* cv_PtrLcv_dnn_CumSumLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::CumSumLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::CumSumLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::CumSumLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::CumSumLayer* cv_PtrLcv_dnn_CumSumLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::CumSumLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::CumSumLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::CumSumLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::CumSumLayer>* cv_PtrLcv_dnn_CumSumLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::CumSumLayer>();
	}

	// cv::Ptr<cv::dnn::CumSumLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::CumSumLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_CumSumLayerG_delete(cv::Ptr<cv::dnn::CumSumLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::CumSumLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::CumSumLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_CumSumLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::CumSumLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::CumSumLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::CumSumLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_CumSumLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::CumSumLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::CumSumLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::CumSumLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::CumSumLayer"]), _)]),
	cv::Ptr<cv::dnn::CumSumLayer>* cv_PtrLcv_dnn_CumSumLayerG_new_const_CumSumLayer(cv::dnn::CumSumLayer* val) {
			return new cv::Ptr<cv::dnn::CumSumLayer>(val);
	}

}

