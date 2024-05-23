extern "C" {
	// cv::Ptr<cv::dnn::MatMulLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::MatMulLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::MatMulLayer* cv_PtrLcv_dnn_MatMulLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::MatMulLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::MatMulLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::MatMulLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::MatMulLayer* cv_PtrLcv_dnn_MatMulLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::MatMulLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::MatMulLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::MatMulLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::MatMulLayer>* cv_PtrLcv_dnn_MatMulLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::MatMulLayer>();
	}

	// cv::Ptr<cv::dnn::MatMulLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::MatMulLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_MatMulLayerG_delete(cv::Ptr<cv::dnn::MatMulLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::MatMulLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::MatMulLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_MatMulLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::MatMulLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::MatMulLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::MatMulLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_MatMulLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::MatMulLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::MatMulLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::MatMulLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::MatMulLayer"]), _)]),
	cv::Ptr<cv::dnn::MatMulLayer>* cv_PtrLcv_dnn_MatMulLayerG_new_const_MatMulLayer(cv::dnn::MatMulLayer* val) {
			return new cv::Ptr<cv::dnn::MatMulLayer>(val);
	}

}

