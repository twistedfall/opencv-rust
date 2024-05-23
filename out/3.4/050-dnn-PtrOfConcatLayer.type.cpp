extern "C" {
	// cv::Ptr<cv::dnn::ConcatLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::ConcatLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::ConcatLayer* cv_PtrLcv_dnn_ConcatLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::ConcatLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ConcatLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::ConcatLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ConcatLayer* cv_PtrLcv_dnn_ConcatLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::ConcatLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ConcatLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::ConcatLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::ConcatLayer>* cv_PtrLcv_dnn_ConcatLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::ConcatLayer>();
	}

	// cv::Ptr<cv::dnn::ConcatLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::ConcatLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_ConcatLayerG_delete(cv::Ptr<cv::dnn::ConcatLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::ConcatLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::ConcatLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_ConcatLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::ConcatLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::ConcatLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::ConcatLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_ConcatLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::ConcatLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::ConcatLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::ConcatLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::ConcatLayer"]), _)]),
	cv::Ptr<cv::dnn::ConcatLayer>* cv_PtrLcv_dnn_ConcatLayerG_new_const_ConcatLayer(cv::dnn::ConcatLayer* val) {
			return new cv::Ptr<cv::dnn::ConcatLayer>(val);
	}

}

