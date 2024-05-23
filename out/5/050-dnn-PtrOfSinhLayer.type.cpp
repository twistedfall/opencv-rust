extern "C" {
	// cv::Ptr<cv::dnn::SinhLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::SinhLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::SinhLayer* cv_PtrLcv_dnn_SinhLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::SinhLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::SinhLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::SinhLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::SinhLayer* cv_PtrLcv_dnn_SinhLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::SinhLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::SinhLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::SinhLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::SinhLayer>* cv_PtrLcv_dnn_SinhLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::SinhLayer>();
	}

	// cv::Ptr<cv::dnn::SinhLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::SinhLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_SinhLayerG_delete(cv::Ptr<cv::dnn::SinhLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::SinhLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::SinhLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_SinhLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::SinhLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::SinhLayer>::to_PtrOfActivationLayer() generated
	// ("cv::Ptr<cv::dnn::SinhLayer>::to_PtrOfActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::ActivationLayer>* cv_PtrLcv_dnn_SinhLayerG_to_PtrOfActivationLayer(cv::Ptr<cv::dnn::SinhLayer>* instance) {
			return new cv::Ptr<cv::dnn::ActivationLayer>(instance->dynamicCast<cv::dnn::ActivationLayer>());
	}

	// cv::Ptr<cv::dnn::SinhLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::SinhLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_SinhLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::SinhLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::SinhLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::SinhLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::SinhLayer"]), _)]),
	cv::Ptr<cv::dnn::SinhLayer>* cv_PtrLcv_dnn_SinhLayerG_new_const_SinhLayer(cv::dnn::SinhLayer* val) {
			return new cv::Ptr<cv::dnn::SinhLayer>(val);
	}

}

