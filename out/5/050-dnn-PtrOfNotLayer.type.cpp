extern "C" {
	// cv::Ptr<cv::dnn::NotLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::NotLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::NotLayer* cv_PtrLcv_dnn_NotLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::NotLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::NotLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::NotLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::NotLayer* cv_PtrLcv_dnn_NotLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::NotLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::NotLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::NotLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::NotLayer>* cv_PtrLcv_dnn_NotLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::NotLayer>();
	}

	// cv::Ptr<cv::dnn::NotLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::NotLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_NotLayerG_delete(cv::Ptr<cv::dnn::NotLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::NotLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::NotLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_NotLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::NotLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::NotLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::NotLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_NotLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::NotLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::NotLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::NotLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::NotLayer"]), _)]),
	cv::Ptr<cv::dnn::NotLayer>* cv_PtrLcv_dnn_NotLayerG_new_const_NotLayer(cv::dnn::NotLayer* val) {
			return new cv::Ptr<cv::dnn::NotLayer>(val);
	}

}

