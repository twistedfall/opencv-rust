extern "C" {
	// cv::Ptr<cv::dnn::ConstLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::ConstLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::ConstLayer* cv_PtrLcv_dnn_ConstLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::ConstLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ConstLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::ConstLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ConstLayer* cv_PtrLcv_dnn_ConstLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::ConstLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ConstLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::ConstLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::ConstLayer>* cv_PtrLcv_dnn_ConstLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::ConstLayer>();
	}

	// cv::Ptr<cv::dnn::ConstLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::ConstLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_ConstLayerG_delete(cv::Ptr<cv::dnn::ConstLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::ConstLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::ConstLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_ConstLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::ConstLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::ConstLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::ConstLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_ConstLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::ConstLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::ConstLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::ConstLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::ConstLayer"]), _)]),
	cv::Ptr<cv::dnn::ConstLayer>* cv_PtrLcv_dnn_ConstLayerG_new_const_ConstLayer(cv::dnn::ConstLayer* val) {
			return new cv::Ptr<cv::dnn::ConstLayer>(val);
	}

}

