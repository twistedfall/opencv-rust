extern "C" {
	// cv::Ptr<cv::dnn::CropLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::CropLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::CropLayer* cv_PtrLcv_dnn_CropLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::CropLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::CropLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::CropLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::CropLayer* cv_PtrLcv_dnn_CropLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::CropLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::CropLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::CropLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::CropLayer>* cv_PtrLcv_dnn_CropLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::CropLayer>();
	}

	// cv::Ptr<cv::dnn::CropLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::CropLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_CropLayerG_delete(cv::Ptr<cv::dnn::CropLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::CropLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::CropLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_CropLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::CropLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::CropLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::CropLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_CropLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::CropLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::CropLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::CropLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::CropLayer"]), _)]),
	cv::Ptr<cv::dnn::CropLayer>* cv_PtrLcv_dnn_CropLayerG_new_const_CropLayer(cv::dnn::CropLayer* val) {
			return new cv::Ptr<cv::dnn::CropLayer>(val);
	}

}

