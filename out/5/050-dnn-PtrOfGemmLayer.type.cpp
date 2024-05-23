extern "C" {
	// cv::Ptr<cv::dnn::GemmLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::GemmLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::GemmLayer* cv_PtrLcv_dnn_GemmLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::GemmLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::GemmLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::GemmLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::GemmLayer* cv_PtrLcv_dnn_GemmLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::GemmLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::GemmLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::GemmLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::GemmLayer>* cv_PtrLcv_dnn_GemmLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::GemmLayer>();
	}

	// cv::Ptr<cv::dnn::GemmLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::GemmLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_GemmLayerG_delete(cv::Ptr<cv::dnn::GemmLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::GemmLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::GemmLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_GemmLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::GemmLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::GemmLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::GemmLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_GemmLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::GemmLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::GemmLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::GemmLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::GemmLayer"]), _)]),
	cv::Ptr<cv::dnn::GemmLayer>* cv_PtrLcv_dnn_GemmLayerG_new_const_GemmLayer(cv::dnn::GemmLayer* val) {
			return new cv::Ptr<cv::dnn::GemmLayer>(val);
	}

}

