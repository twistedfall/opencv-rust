extern "C" {
	// cv::Ptr<cv::dnn::SoftmaxLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::SoftmaxLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::SoftmaxLayer* cv_PtrLcv_dnn_SoftmaxLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::SoftmaxLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::SoftmaxLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::SoftmaxLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::SoftmaxLayer* cv_PtrLcv_dnn_SoftmaxLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::SoftmaxLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::SoftmaxLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::SoftmaxLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::SoftmaxLayer>* cv_PtrLcv_dnn_SoftmaxLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::SoftmaxLayer>();
	}

	// cv::Ptr<cv::dnn::SoftmaxLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::SoftmaxLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_SoftmaxLayerG_delete(cv::Ptr<cv::dnn::SoftmaxLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::SoftmaxLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::SoftmaxLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_SoftmaxLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::SoftmaxLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::SoftmaxLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::SoftmaxLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_SoftmaxLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::SoftmaxLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::SoftmaxLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::SoftmaxLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::SoftmaxLayer"]), _)]),
	cv::Ptr<cv::dnn::SoftmaxLayer>* cv_PtrLcv_dnn_SoftmaxLayerG_new_const_SoftmaxLayer(cv::dnn::SoftmaxLayer* val) {
			return new cv::Ptr<cv::dnn::SoftmaxLayer>(val);
	}

}

