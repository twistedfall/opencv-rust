extern "C" {
	// cv::Ptr<cv::dnn::DepthToSpaceLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::DepthToSpaceLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::DepthToSpaceLayer* cv_PtrLcv_dnn_DepthToSpaceLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::DepthToSpaceLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::DepthToSpaceLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::DepthToSpaceLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::DepthToSpaceLayer* cv_PtrLcv_dnn_DepthToSpaceLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::DepthToSpaceLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::DepthToSpaceLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::DepthToSpaceLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::DepthToSpaceLayer>* cv_PtrLcv_dnn_DepthToSpaceLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::DepthToSpaceLayer>();
	}

	// cv::Ptr<cv::dnn::DepthToSpaceLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::DepthToSpaceLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_DepthToSpaceLayerG_delete(cv::Ptr<cv::dnn::DepthToSpaceLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::DepthToSpaceLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::DepthToSpaceLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_DepthToSpaceLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::DepthToSpaceLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::DepthToSpaceLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::DepthToSpaceLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_DepthToSpaceLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::DepthToSpaceLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::DepthToSpaceLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::DepthToSpaceLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::DepthToSpaceLayer"]), _)]),
	cv::Ptr<cv::dnn::DepthToSpaceLayer>* cv_PtrLcv_dnn_DepthToSpaceLayerG_new_const_DepthToSpaceLayer(cv::dnn::DepthToSpaceLayer* val) {
			return new cv::Ptr<cv::dnn::DepthToSpaceLayer>(val);
	}

}

