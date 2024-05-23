extern "C" {
	// cv::Ptr<cv::dnn::ReduceLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::ReduceLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::ReduceLayer* cv_PtrLcv_dnn_ReduceLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::ReduceLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ReduceLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::ReduceLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ReduceLayer* cv_PtrLcv_dnn_ReduceLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::ReduceLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ReduceLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::ReduceLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::ReduceLayer>* cv_PtrLcv_dnn_ReduceLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::ReduceLayer>();
	}

	// cv::Ptr<cv::dnn::ReduceLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::ReduceLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_ReduceLayerG_delete(cv::Ptr<cv::dnn::ReduceLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::ReduceLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::ReduceLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_ReduceLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::ReduceLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::ReduceLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::ReduceLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_ReduceLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::ReduceLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::ReduceLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::ReduceLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::ReduceLayer"]), _)]),
	cv::Ptr<cv::dnn::ReduceLayer>* cv_PtrLcv_dnn_ReduceLayerG_new_const_ReduceLayer(cv::dnn::ReduceLayer* val) {
			return new cv::Ptr<cv::dnn::ReduceLayer>(val);
	}

}

