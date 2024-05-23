extern "C" {
	// cv::Ptr<cv::dnn::CompareLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::CompareLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::CompareLayer* cv_PtrLcv_dnn_CompareLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::CompareLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::CompareLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::CompareLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::CompareLayer* cv_PtrLcv_dnn_CompareLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::CompareLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::CompareLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::CompareLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::CompareLayer>* cv_PtrLcv_dnn_CompareLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::CompareLayer>();
	}

	// cv::Ptr<cv::dnn::CompareLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::CompareLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_CompareLayerG_delete(cv::Ptr<cv::dnn::CompareLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::CompareLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::CompareLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_CompareLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::CompareLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::CompareLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::CompareLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_CompareLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::CompareLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::CompareLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::CompareLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::CompareLayer"]), _)]),
	cv::Ptr<cv::dnn::CompareLayer>* cv_PtrLcv_dnn_CompareLayerG_new_const_CompareLayer(cv::dnn::CompareLayer* val) {
			return new cv::Ptr<cv::dnn::CompareLayer>(val);
	}

}

