extern "C" {
	// cv::Ptr<cv::dnn::MVNLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::MVNLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::MVNLayer* cv_PtrLcv_dnn_MVNLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::MVNLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::MVNLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::MVNLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::MVNLayer* cv_PtrLcv_dnn_MVNLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::MVNLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::MVNLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::MVNLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::MVNLayer>* cv_PtrLcv_dnn_MVNLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::MVNLayer>();
	}

	// cv::Ptr<cv::dnn::MVNLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::MVNLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_MVNLayerG_delete(cv::Ptr<cv::dnn::MVNLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::MVNLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::MVNLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_MVNLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::MVNLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::MVNLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::MVNLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_MVNLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::MVNLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::MVNLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::MVNLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::MVNLayer"]), _)]),
	cv::Ptr<cv::dnn::MVNLayer>* cv_PtrLcv_dnn_MVNLayerG_new_const_MVNLayer(cv::dnn::MVNLayer* val) {
			return new cv::Ptr<cv::dnn::MVNLayer>(val);
	}

}

