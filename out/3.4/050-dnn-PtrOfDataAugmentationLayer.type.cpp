extern "C" {
	// cv::Ptr<cv::dnn::DataAugmentationLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::DataAugmentationLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::DataAugmentationLayer* cv_PtrLcv_dnn_DataAugmentationLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::DataAugmentationLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::DataAugmentationLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::DataAugmentationLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::DataAugmentationLayer* cv_PtrLcv_dnn_DataAugmentationLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::DataAugmentationLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::DataAugmentationLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::DataAugmentationLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::DataAugmentationLayer>* cv_PtrLcv_dnn_DataAugmentationLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::DataAugmentationLayer>();
	}

	// cv::Ptr<cv::dnn::DataAugmentationLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::DataAugmentationLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_DataAugmentationLayerG_delete(cv::Ptr<cv::dnn::DataAugmentationLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::DataAugmentationLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::DataAugmentationLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_DataAugmentationLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::DataAugmentationLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::DataAugmentationLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::DataAugmentationLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_DataAugmentationLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::DataAugmentationLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::DataAugmentationLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::DataAugmentationLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::DataAugmentationLayer"]), _)]),
	cv::Ptr<cv::dnn::DataAugmentationLayer>* cv_PtrLcv_dnn_DataAugmentationLayerG_new_const_DataAugmentationLayer(cv::dnn::DataAugmentationLayer* val) {
			return new cv::Ptr<cv::dnn::DataAugmentationLayer>(val);
	}

}

