extern "C" {
	// cv::Ptr<cv::dnn::CastLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::CastLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::CastLayer* cv_PtrLcv_dnn_CastLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::CastLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::CastLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::CastLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::CastLayer* cv_PtrLcv_dnn_CastLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::CastLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::CastLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::CastLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::CastLayer>* cv_PtrLcv_dnn_CastLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::CastLayer>();
	}

	// cv::Ptr<cv::dnn::CastLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::CastLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_CastLayerG_delete(cv::Ptr<cv::dnn::CastLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::CastLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::CastLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_CastLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::CastLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::CastLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::CastLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_CastLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::CastLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::CastLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::CastLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::CastLayer"]), _)]),
	cv::Ptr<cv::dnn::CastLayer>* cv_PtrLcv_dnn_CastLayerG_new_const_CastLayer(cv::dnn::CastLayer* val) {
			return new cv::Ptr<cv::dnn::CastLayer>(val);
	}

}

