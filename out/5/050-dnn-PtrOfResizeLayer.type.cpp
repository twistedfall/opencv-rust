extern "C" {
	// cv::Ptr<cv::dnn::ResizeLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::ResizeLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::ResizeLayer* cv_PtrLcv_dnn_ResizeLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::ResizeLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ResizeLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::ResizeLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ResizeLayer* cv_PtrLcv_dnn_ResizeLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::ResizeLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ResizeLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::ResizeLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::ResizeLayer>* cv_PtrLcv_dnn_ResizeLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::ResizeLayer>();
	}

	// cv::Ptr<cv::dnn::ResizeLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::ResizeLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_ResizeLayerG_delete(cv::Ptr<cv::dnn::ResizeLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::ResizeLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::ResizeLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_ResizeLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::ResizeLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::ResizeLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::ResizeLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_ResizeLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::ResizeLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::ResizeLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::ResizeLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::ResizeLayer"]), _)]),
	cv::Ptr<cv::dnn::ResizeLayer>* cv_PtrLcv_dnn_ResizeLayerG_new_const_ResizeLayer(cv::dnn::ResizeLayer* val) {
			return new cv::Ptr<cv::dnn::ResizeLayer>(val);
	}

}

