extern "C" {
	// cv::Ptr<cv::dnn::NaryEltwiseLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::NaryEltwiseLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::NaryEltwiseLayer* cv_PtrLcv_dnn_NaryEltwiseLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::NaryEltwiseLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::NaryEltwiseLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::NaryEltwiseLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::NaryEltwiseLayer* cv_PtrLcv_dnn_NaryEltwiseLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::NaryEltwiseLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::NaryEltwiseLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::NaryEltwiseLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::NaryEltwiseLayer>* cv_PtrLcv_dnn_NaryEltwiseLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::NaryEltwiseLayer>();
	}

	// cv::Ptr<cv::dnn::NaryEltwiseLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::NaryEltwiseLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_NaryEltwiseLayerG_delete(cv::Ptr<cv::dnn::NaryEltwiseLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::NaryEltwiseLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::NaryEltwiseLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_NaryEltwiseLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::NaryEltwiseLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::NaryEltwiseLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::NaryEltwiseLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_NaryEltwiseLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::NaryEltwiseLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::NaryEltwiseLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::NaryEltwiseLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::NaryEltwiseLayer"]), _)]),
	cv::Ptr<cv::dnn::NaryEltwiseLayer>* cv_PtrLcv_dnn_NaryEltwiseLayerG_new_const_NaryEltwiseLayer(cv::dnn::NaryEltwiseLayer* val) {
			return new cv::Ptr<cv::dnn::NaryEltwiseLayer>(val);
	}

}

