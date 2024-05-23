extern "C" {
	// cv::Ptr<cv::dnn::ShrinkLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::ShrinkLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::ShrinkLayer* cv_PtrLcv_dnn_ShrinkLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::ShrinkLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ShrinkLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::ShrinkLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ShrinkLayer* cv_PtrLcv_dnn_ShrinkLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::ShrinkLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ShrinkLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::ShrinkLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::ShrinkLayer>* cv_PtrLcv_dnn_ShrinkLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::ShrinkLayer>();
	}

	// cv::Ptr<cv::dnn::ShrinkLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::ShrinkLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_ShrinkLayerG_delete(cv::Ptr<cv::dnn::ShrinkLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::ShrinkLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::ShrinkLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_ShrinkLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::ShrinkLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::ShrinkLayer>::to_PtrOfActivationLayer() generated
	// ("cv::Ptr<cv::dnn::ShrinkLayer>::to_PtrOfActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::ActivationLayer>* cv_PtrLcv_dnn_ShrinkLayerG_to_PtrOfActivationLayer(cv::Ptr<cv::dnn::ShrinkLayer>* instance) {
			return new cv::Ptr<cv::dnn::ActivationLayer>(instance->dynamicCast<cv::dnn::ActivationLayer>());
	}

	// cv::Ptr<cv::dnn::ShrinkLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::ShrinkLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_ShrinkLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::ShrinkLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::ShrinkLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::ShrinkLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::ShrinkLayer"]), _)]),
	cv::Ptr<cv::dnn::ShrinkLayer>* cv_PtrLcv_dnn_ShrinkLayerG_new_const_ShrinkLayer(cv::dnn::ShrinkLayer* val) {
			return new cv::Ptr<cv::dnn::ShrinkLayer>(val);
	}

}

