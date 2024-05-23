extern "C" {
	// cv::Ptr<cv::dnn::AcoshLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::AcoshLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::AcoshLayer* cv_PtrLcv_dnn_AcoshLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::AcoshLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::AcoshLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::AcoshLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::AcoshLayer* cv_PtrLcv_dnn_AcoshLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::AcoshLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::AcoshLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::AcoshLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::AcoshLayer>* cv_PtrLcv_dnn_AcoshLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::AcoshLayer>();
	}

	// cv::Ptr<cv::dnn::AcoshLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::AcoshLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_AcoshLayerG_delete(cv::Ptr<cv::dnn::AcoshLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::AcoshLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::AcoshLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_AcoshLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::AcoshLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::AcoshLayer>::to_PtrOfActivationLayer() generated
	// ("cv::Ptr<cv::dnn::AcoshLayer>::to_PtrOfActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::ActivationLayer>* cv_PtrLcv_dnn_AcoshLayerG_to_PtrOfActivationLayer(cv::Ptr<cv::dnn::AcoshLayer>* instance) {
			return new cv::Ptr<cv::dnn::ActivationLayer>(instance->dynamicCast<cv::dnn::ActivationLayer>());
	}

	// cv::Ptr<cv::dnn::AcoshLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::AcoshLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_AcoshLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::AcoshLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::AcoshLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::AcoshLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::AcoshLayer"]), _)]),
	cv::Ptr<cv::dnn::AcoshLayer>* cv_PtrLcv_dnn_AcoshLayerG_new_const_AcoshLayer(cv::dnn::AcoshLayer* val) {
			return new cv::Ptr<cv::dnn::AcoshLayer>(val);
	}

}

