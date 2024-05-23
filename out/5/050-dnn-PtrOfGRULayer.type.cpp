extern "C" {
	// cv::Ptr<cv::dnn::GRULayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::GRULayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::GRULayer* cv_PtrLcv_dnn_GRULayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::GRULayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::GRULayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::GRULayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::GRULayer* cv_PtrLcv_dnn_GRULayerG_getInnerPtrMut(cv::Ptr<cv::dnn::GRULayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::GRULayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::GRULayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::GRULayer>* cv_PtrLcv_dnn_GRULayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::GRULayer>();
	}

	// cv::Ptr<cv::dnn::GRULayer>::delete() generated
	// ("cv::Ptr<cv::dnn::GRULayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_GRULayerG_delete(cv::Ptr<cv::dnn::GRULayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::GRULayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::GRULayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_GRULayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::GRULayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::GRULayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::GRULayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_GRULayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::GRULayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::GRULayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::GRULayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::GRULayer"]), _)]),
	cv::Ptr<cv::dnn::GRULayer>* cv_PtrLcv_dnn_GRULayerG_new_const_GRULayer(cv::dnn::GRULayer* val) {
			return new cv::Ptr<cv::dnn::GRULayer>(val);
	}

}

