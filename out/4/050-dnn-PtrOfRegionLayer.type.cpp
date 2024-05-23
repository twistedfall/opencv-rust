extern "C" {
	// cv::Ptr<cv::dnn::RegionLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::RegionLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::RegionLayer* cv_PtrLcv_dnn_RegionLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::RegionLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::RegionLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::RegionLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::RegionLayer* cv_PtrLcv_dnn_RegionLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::RegionLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::RegionLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::RegionLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::RegionLayer>* cv_PtrLcv_dnn_RegionLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::RegionLayer>();
	}

	// cv::Ptr<cv::dnn::RegionLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::RegionLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_RegionLayerG_delete(cv::Ptr<cv::dnn::RegionLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::RegionLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::RegionLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_RegionLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::RegionLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::RegionLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::RegionLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_RegionLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::RegionLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::RegionLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::RegionLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::RegionLayer"]), _)]),
	cv::Ptr<cv::dnn::RegionLayer>* cv_PtrLcv_dnn_RegionLayerG_new_const_RegionLayer(cv::dnn::RegionLayer* val) {
			return new cv::Ptr<cv::dnn::RegionLayer>(val);
	}

}

