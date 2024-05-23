extern "C" {
	// cv::Ptr<cv::dnn::ShiftLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::ShiftLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::ShiftLayer* cv_PtrLcv_dnn_ShiftLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::ShiftLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ShiftLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::ShiftLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ShiftLayer* cv_PtrLcv_dnn_ShiftLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::ShiftLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ShiftLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::ShiftLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::ShiftLayer>* cv_PtrLcv_dnn_ShiftLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::ShiftLayer>();
	}

	// cv::Ptr<cv::dnn::ShiftLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::ShiftLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_ShiftLayerG_delete(cv::Ptr<cv::dnn::ShiftLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::ShiftLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::ShiftLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_ShiftLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::ShiftLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::ShiftLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::ShiftLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_ShiftLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::ShiftLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::ShiftLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::ShiftLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::ShiftLayer"]), _)]),
	cv::Ptr<cv::dnn::ShiftLayer>* cv_PtrLcv_dnn_ShiftLayerG_new_const_ShiftLayer(cv::dnn::ShiftLayer* val) {
			return new cv::Ptr<cv::dnn::ShiftLayer>(val);
	}

}

