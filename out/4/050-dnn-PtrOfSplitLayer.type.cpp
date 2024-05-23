extern "C" {
	// cv::Ptr<cv::dnn::SplitLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::SplitLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::SplitLayer* cv_PtrLcv_dnn_SplitLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::SplitLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::SplitLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::SplitLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::SplitLayer* cv_PtrLcv_dnn_SplitLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::SplitLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::SplitLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::SplitLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::SplitLayer>* cv_PtrLcv_dnn_SplitLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::SplitLayer>();
	}

	// cv::Ptr<cv::dnn::SplitLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::SplitLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_SplitLayerG_delete(cv::Ptr<cv::dnn::SplitLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::SplitLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::SplitLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_SplitLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::SplitLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::SplitLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::SplitLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_SplitLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::SplitLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::SplitLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::SplitLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::SplitLayer"]), _)]),
	cv::Ptr<cv::dnn::SplitLayer>* cv_PtrLcv_dnn_SplitLayerG_new_const_SplitLayer(cv::dnn::SplitLayer* val) {
			return new cv::Ptr<cv::dnn::SplitLayer>(val);
	}

}

