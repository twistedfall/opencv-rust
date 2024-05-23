extern "C" {
	// cv::Ptr<cv::dnn::TransposeLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::TransposeLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::TransposeLayer* cv_PtrLcv_dnn_TransposeLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::TransposeLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::TransposeLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::TransposeLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::TransposeLayer* cv_PtrLcv_dnn_TransposeLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::TransposeLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::TransposeLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::TransposeLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::TransposeLayer>* cv_PtrLcv_dnn_TransposeLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::TransposeLayer>();
	}

	// cv::Ptr<cv::dnn::TransposeLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::TransposeLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_TransposeLayerG_delete(cv::Ptr<cv::dnn::TransposeLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::TransposeLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::TransposeLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_TransposeLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::TransposeLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::TransposeLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::TransposeLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_TransposeLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::TransposeLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::TransposeLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::TransposeLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::TransposeLayer"]), _)]),
	cv::Ptr<cv::dnn::TransposeLayer>* cv_PtrLcv_dnn_TransposeLayerG_new_const_TransposeLayer(cv::dnn::TransposeLayer* val) {
			return new cv::Ptr<cv::dnn::TransposeLayer>(val);
	}

}

