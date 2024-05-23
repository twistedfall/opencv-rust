extern "C" {
	// cv::Ptr<cv::dnn::AccumLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::AccumLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::AccumLayer* cv_PtrLcv_dnn_AccumLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::AccumLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::AccumLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::AccumLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::AccumLayer* cv_PtrLcv_dnn_AccumLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::AccumLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::AccumLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::AccumLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::AccumLayer>* cv_PtrLcv_dnn_AccumLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::AccumLayer>();
	}

	// cv::Ptr<cv::dnn::AccumLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::AccumLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_AccumLayerG_delete(cv::Ptr<cv::dnn::AccumLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::AccumLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::AccumLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_AccumLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::AccumLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::AccumLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::AccumLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_AccumLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::AccumLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::AccumLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::AccumLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::AccumLayer"]), _)]),
	cv::Ptr<cv::dnn::AccumLayer>* cv_PtrLcv_dnn_AccumLayerG_new_const_AccumLayer(cv::dnn::AccumLayer* val) {
			return new cv::Ptr<cv::dnn::AccumLayer>(val);
	}

}

