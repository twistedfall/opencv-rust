extern "C" {
	// cv::Ptr<cv::dnn::SoftplusLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::SoftplusLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::SoftplusLayer* cv_PtrLcv_dnn_SoftplusLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::SoftplusLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::SoftplusLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::SoftplusLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::SoftplusLayer* cv_PtrLcv_dnn_SoftplusLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::SoftplusLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::SoftplusLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::SoftplusLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::SoftplusLayer>* cv_PtrLcv_dnn_SoftplusLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::SoftplusLayer>();
	}

	// cv::Ptr<cv::dnn::SoftplusLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::SoftplusLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_SoftplusLayerG_delete(cv::Ptr<cv::dnn::SoftplusLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::SoftplusLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::SoftplusLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_SoftplusLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::SoftplusLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::SoftplusLayer>::to_PtrOfActivationLayer() generated
	// ("cv::Ptr<cv::dnn::SoftplusLayer>::to_PtrOfActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::ActivationLayer>* cv_PtrLcv_dnn_SoftplusLayerG_to_PtrOfActivationLayer(cv::Ptr<cv::dnn::SoftplusLayer>* instance) {
			return new cv::Ptr<cv::dnn::ActivationLayer>(instance->dynamicCast<cv::dnn::ActivationLayer>());
	}

	// cv::Ptr<cv::dnn::SoftplusLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::SoftplusLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_SoftplusLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::SoftplusLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::SoftplusLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::SoftplusLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::SoftplusLayer"]), _)]),
	cv::Ptr<cv::dnn::SoftplusLayer>* cv_PtrLcv_dnn_SoftplusLayerG_new_const_SoftplusLayer(cv::dnn::SoftplusLayer* val) {
			return new cv::Ptr<cv::dnn::SoftplusLayer>(val);
	}

}

