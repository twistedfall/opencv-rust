extern "C" {
	// cv::Ptr<cv::dnn::InnerProductLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::InnerProductLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::InnerProductLayer* cv_PtrLcv_dnn_InnerProductLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::InnerProductLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::InnerProductLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::InnerProductLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::InnerProductLayer* cv_PtrLcv_dnn_InnerProductLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::InnerProductLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::InnerProductLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::InnerProductLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::InnerProductLayer>* cv_PtrLcv_dnn_InnerProductLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::InnerProductLayer>();
	}

	// cv::Ptr<cv::dnn::InnerProductLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::InnerProductLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_InnerProductLayerG_delete(cv::Ptr<cv::dnn::InnerProductLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::InnerProductLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::InnerProductLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_InnerProductLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::InnerProductLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::InnerProductLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::InnerProductLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_InnerProductLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::InnerProductLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::InnerProductLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::InnerProductLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::InnerProductLayer"]), _)]),
	cv::Ptr<cv::dnn::InnerProductLayer>* cv_PtrLcv_dnn_InnerProductLayerG_new_const_InnerProductLayer(cv::dnn::InnerProductLayer* val) {
			return new cv::Ptr<cv::dnn::InnerProductLayer>(val);
	}

}

