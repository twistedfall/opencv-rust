extern "C" {
	// cv::Ptr<cv::dnn::InstanceNormLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::InstanceNormLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::InstanceNormLayer* cv_PtrLcv_dnn_InstanceNormLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::InstanceNormLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::InstanceNormLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::InstanceNormLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::InstanceNormLayer* cv_PtrLcv_dnn_InstanceNormLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::InstanceNormLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::InstanceNormLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::InstanceNormLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::InstanceNormLayer>* cv_PtrLcv_dnn_InstanceNormLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::InstanceNormLayer>();
	}

	// cv::Ptr<cv::dnn::InstanceNormLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::InstanceNormLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_InstanceNormLayerG_delete(cv::Ptr<cv::dnn::InstanceNormLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::InstanceNormLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::InstanceNormLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_InstanceNormLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::InstanceNormLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::InstanceNormLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::InstanceNormLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_InstanceNormLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::InstanceNormLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::InstanceNormLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::InstanceNormLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::InstanceNormLayer"]), _)]),
	cv::Ptr<cv::dnn::InstanceNormLayer>* cv_PtrLcv_dnn_InstanceNormLayerG_new_const_InstanceNormLayer(cv::dnn::InstanceNormLayer* val) {
			return new cv::Ptr<cv::dnn::InstanceNormLayer>(val);
	}

}

