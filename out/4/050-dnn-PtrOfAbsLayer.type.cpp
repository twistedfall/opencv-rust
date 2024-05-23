extern "C" {
	// cv::Ptr<cv::dnn::AbsLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::AbsLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::AbsLayer* cv_PtrLcv_dnn_AbsLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::AbsLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::AbsLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::AbsLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::AbsLayer* cv_PtrLcv_dnn_AbsLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::AbsLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::AbsLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::AbsLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::AbsLayer>* cv_PtrLcv_dnn_AbsLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::AbsLayer>();
	}

	// cv::Ptr<cv::dnn::AbsLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::AbsLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_AbsLayerG_delete(cv::Ptr<cv::dnn::AbsLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::AbsLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::AbsLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_AbsLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::AbsLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::AbsLayer>::to_PtrOfActivationLayer() generated
	// ("cv::Ptr<cv::dnn::AbsLayer>::to_PtrOfActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::ActivationLayer>* cv_PtrLcv_dnn_AbsLayerG_to_PtrOfActivationLayer(cv::Ptr<cv::dnn::AbsLayer>* instance) {
			return new cv::Ptr<cv::dnn::ActivationLayer>(instance->dynamicCast<cv::dnn::ActivationLayer>());
	}

	// cv::Ptr<cv::dnn::AbsLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::AbsLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_AbsLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::AbsLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::AbsLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::AbsLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::AbsLayer"]), _)]),
	cv::Ptr<cv::dnn::AbsLayer>* cv_PtrLcv_dnn_AbsLayerG_new_const_AbsLayer(cv::dnn::AbsLayer* val) {
			return new cv::Ptr<cv::dnn::AbsLayer>(val);
	}

}

