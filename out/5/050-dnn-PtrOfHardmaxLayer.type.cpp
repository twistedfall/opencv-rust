extern "C" {
	// cv::Ptr<cv::dnn::HardmaxLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::HardmaxLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::HardmaxLayer* cv_PtrLcv_dnn_HardmaxLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::HardmaxLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::HardmaxLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::HardmaxLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::HardmaxLayer* cv_PtrLcv_dnn_HardmaxLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::HardmaxLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::HardmaxLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::HardmaxLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::HardmaxLayer>* cv_PtrLcv_dnn_HardmaxLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::HardmaxLayer>();
	}

	// cv::Ptr<cv::dnn::HardmaxLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::HardmaxLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_HardmaxLayerG_delete(cv::Ptr<cv::dnn::HardmaxLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::HardmaxLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::HardmaxLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_HardmaxLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::HardmaxLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::HardmaxLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::HardmaxLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_HardmaxLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::HardmaxLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::HardmaxLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::HardmaxLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::HardmaxLayer"]), _)]),
	cv::Ptr<cv::dnn::HardmaxLayer>* cv_PtrLcv_dnn_HardmaxLayerG_new_const_HardmaxLayer(cv::dnn::HardmaxLayer* val) {
			return new cv::Ptr<cv::dnn::HardmaxLayer>(val);
	}

}

