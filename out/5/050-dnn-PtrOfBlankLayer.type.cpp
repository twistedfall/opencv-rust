extern "C" {
	// cv::Ptr<cv::dnn::BlankLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::BlankLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::BlankLayer* cv_PtrLcv_dnn_BlankLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::BlankLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::BlankLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::BlankLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::BlankLayer* cv_PtrLcv_dnn_BlankLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::BlankLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::BlankLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::BlankLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::BlankLayer>* cv_PtrLcv_dnn_BlankLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::BlankLayer>();
	}

	// cv::Ptr<cv::dnn::BlankLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::BlankLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_BlankLayerG_delete(cv::Ptr<cv::dnn::BlankLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::BlankLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::BlankLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_BlankLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::BlankLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::BlankLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::BlankLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_BlankLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::BlankLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::BlankLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::BlankLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::BlankLayer"]), _)]),
	cv::Ptr<cv::dnn::BlankLayer>* cv_PtrLcv_dnn_BlankLayerG_new_const_BlankLayer(cv::dnn::BlankLayer* val) {
			return new cv::Ptr<cv::dnn::BlankLayer>(val);
	}

}

