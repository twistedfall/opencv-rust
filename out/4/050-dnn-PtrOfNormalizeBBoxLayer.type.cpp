extern "C" {
	// cv::Ptr<cv::dnn::NormalizeBBoxLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::NormalizeBBoxLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::NormalizeBBoxLayer* cv_PtrLcv_dnn_NormalizeBBoxLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::NormalizeBBoxLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::NormalizeBBoxLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::NormalizeBBoxLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::NormalizeBBoxLayer* cv_PtrLcv_dnn_NormalizeBBoxLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::NormalizeBBoxLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::NormalizeBBoxLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::NormalizeBBoxLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::NormalizeBBoxLayer>* cv_PtrLcv_dnn_NormalizeBBoxLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::NormalizeBBoxLayer>();
	}

	// cv::Ptr<cv::dnn::NormalizeBBoxLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::NormalizeBBoxLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_NormalizeBBoxLayerG_delete(cv::Ptr<cv::dnn::NormalizeBBoxLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::NormalizeBBoxLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::NormalizeBBoxLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_NormalizeBBoxLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::NormalizeBBoxLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::NormalizeBBoxLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::NormalizeBBoxLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_NormalizeBBoxLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::NormalizeBBoxLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::NormalizeBBoxLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::NormalizeBBoxLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::NormalizeBBoxLayer"]), _)]),
	cv::Ptr<cv::dnn::NormalizeBBoxLayer>* cv_PtrLcv_dnn_NormalizeBBoxLayerG_new_const_NormalizeBBoxLayer(cv::dnn::NormalizeBBoxLayer* val) {
			return new cv::Ptr<cv::dnn::NormalizeBBoxLayer>(val);
	}

}

