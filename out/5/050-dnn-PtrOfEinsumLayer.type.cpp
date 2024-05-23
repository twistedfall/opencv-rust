extern "C" {
	// cv::Ptr<cv::dnn::EinsumLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::EinsumLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::EinsumLayer* cv_PtrLcv_dnn_EinsumLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::EinsumLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::EinsumLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::EinsumLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::EinsumLayer* cv_PtrLcv_dnn_EinsumLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::EinsumLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::EinsumLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::EinsumLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::EinsumLayer>* cv_PtrLcv_dnn_EinsumLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::EinsumLayer>();
	}

	// cv::Ptr<cv::dnn::EinsumLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::EinsumLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_EinsumLayerG_delete(cv::Ptr<cv::dnn::EinsumLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::EinsumLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::EinsumLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_EinsumLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::EinsumLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::EinsumLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::EinsumLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_EinsumLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::EinsumLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::EinsumLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::EinsumLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::EinsumLayer"]), _)]),
	cv::Ptr<cv::dnn::EinsumLayer>* cv_PtrLcv_dnn_EinsumLayerG_new_const_EinsumLayer(cv::dnn::EinsumLayer* val) {
			return new cv::Ptr<cv::dnn::EinsumLayer>(val);
	}

}

