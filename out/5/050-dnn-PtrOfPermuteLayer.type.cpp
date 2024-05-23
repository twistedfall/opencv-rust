extern "C" {
	// cv::Ptr<cv::dnn::PermuteLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::PermuteLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::PermuteLayer* cv_PtrLcv_dnn_PermuteLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::PermuteLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::PermuteLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::PermuteLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::PermuteLayer* cv_PtrLcv_dnn_PermuteLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::PermuteLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::PermuteLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::PermuteLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::PermuteLayer>* cv_PtrLcv_dnn_PermuteLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::PermuteLayer>();
	}

	// cv::Ptr<cv::dnn::PermuteLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::PermuteLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_PermuteLayerG_delete(cv::Ptr<cv::dnn::PermuteLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::PermuteLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::PermuteLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_PermuteLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::PermuteLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::PermuteLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::PermuteLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_PermuteLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::PermuteLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::PermuteLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::PermuteLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::PermuteLayer"]), _)]),
	cv::Ptr<cv::dnn::PermuteLayer>* cv_PtrLcv_dnn_PermuteLayerG_new_const_PermuteLayer(cv::dnn::PermuteLayer* val) {
			return new cv::Ptr<cv::dnn::PermuteLayer>(val);
	}

}

