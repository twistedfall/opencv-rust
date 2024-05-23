extern "C" {
	// cv::Ptr<cv::dnn::ArgLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::ArgLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::ArgLayer* cv_PtrLcv_dnn_ArgLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::ArgLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ArgLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::ArgLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ArgLayer* cv_PtrLcv_dnn_ArgLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::ArgLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ArgLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::ArgLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::ArgLayer>* cv_PtrLcv_dnn_ArgLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::ArgLayer>();
	}

	// cv::Ptr<cv::dnn::ArgLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::ArgLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_ArgLayerG_delete(cv::Ptr<cv::dnn::ArgLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::ArgLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::ArgLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_ArgLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::ArgLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::ArgLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::ArgLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_ArgLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::ArgLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::ArgLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::ArgLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::ArgLayer"]), _)]),
	cv::Ptr<cv::dnn::ArgLayer>* cv_PtrLcv_dnn_ArgLayerG_new_const_ArgLayer(cv::dnn::ArgLayer* val) {
			return new cv::Ptr<cv::dnn::ArgLayer>(val);
	}

}

