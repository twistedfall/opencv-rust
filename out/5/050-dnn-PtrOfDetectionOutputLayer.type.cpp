extern "C" {
	// cv::Ptr<cv::dnn::DetectionOutputLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::DetectionOutputLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::DetectionOutputLayer* cv_PtrLcv_dnn_DetectionOutputLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::DetectionOutputLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::DetectionOutputLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::DetectionOutputLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::DetectionOutputLayer* cv_PtrLcv_dnn_DetectionOutputLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::DetectionOutputLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::DetectionOutputLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::DetectionOutputLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::DetectionOutputLayer>* cv_PtrLcv_dnn_DetectionOutputLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::DetectionOutputLayer>();
	}

	// cv::Ptr<cv::dnn::DetectionOutputLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::DetectionOutputLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_DetectionOutputLayerG_delete(cv::Ptr<cv::dnn::DetectionOutputLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::DetectionOutputLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::DetectionOutputLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_DetectionOutputLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::DetectionOutputLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::DetectionOutputLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::DetectionOutputLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_DetectionOutputLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::DetectionOutputLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::DetectionOutputLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::DetectionOutputLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::DetectionOutputLayer"]), _)]),
	cv::Ptr<cv::dnn::DetectionOutputLayer>* cv_PtrLcv_dnn_DetectionOutputLayerG_new_const_DetectionOutputLayer(cv::dnn::DetectionOutputLayer* val) {
			return new cv::Ptr<cv::dnn::DetectionOutputLayer>(val);
	}

}

