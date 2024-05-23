extern "C" {
	// cv::Ptr<cv::dnn::MaxUnpoolLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::MaxUnpoolLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::MaxUnpoolLayer* cv_PtrLcv_dnn_MaxUnpoolLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::MaxUnpoolLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::MaxUnpoolLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::MaxUnpoolLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::MaxUnpoolLayer* cv_PtrLcv_dnn_MaxUnpoolLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::MaxUnpoolLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::MaxUnpoolLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::MaxUnpoolLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::MaxUnpoolLayer>* cv_PtrLcv_dnn_MaxUnpoolLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::MaxUnpoolLayer>();
	}

	// cv::Ptr<cv::dnn::MaxUnpoolLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::MaxUnpoolLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_MaxUnpoolLayerG_delete(cv::Ptr<cv::dnn::MaxUnpoolLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::MaxUnpoolLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::MaxUnpoolLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_MaxUnpoolLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::MaxUnpoolLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::MaxUnpoolLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::MaxUnpoolLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_MaxUnpoolLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::MaxUnpoolLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::MaxUnpoolLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::MaxUnpoolLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::MaxUnpoolLayer"]), _)]),
	cv::Ptr<cv::dnn::MaxUnpoolLayer>* cv_PtrLcv_dnn_MaxUnpoolLayerG_new_const_MaxUnpoolLayer(cv::dnn::MaxUnpoolLayer* val) {
			return new cv::Ptr<cv::dnn::MaxUnpoolLayer>(val);
	}

}

