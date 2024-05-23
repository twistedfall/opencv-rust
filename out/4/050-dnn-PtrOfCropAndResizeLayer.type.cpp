extern "C" {
	// cv::Ptr<cv::dnn::CropAndResizeLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::CropAndResizeLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::CropAndResizeLayer* cv_PtrLcv_dnn_CropAndResizeLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::CropAndResizeLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::CropAndResizeLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::CropAndResizeLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::CropAndResizeLayer* cv_PtrLcv_dnn_CropAndResizeLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::CropAndResizeLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::CropAndResizeLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::CropAndResizeLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::CropAndResizeLayer>* cv_PtrLcv_dnn_CropAndResizeLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::CropAndResizeLayer>();
	}

	// cv::Ptr<cv::dnn::CropAndResizeLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::CropAndResizeLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_CropAndResizeLayerG_delete(cv::Ptr<cv::dnn::CropAndResizeLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::CropAndResizeLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::CropAndResizeLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_CropAndResizeLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::CropAndResizeLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::CropAndResizeLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::CropAndResizeLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_CropAndResizeLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::CropAndResizeLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::CropAndResizeLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::CropAndResizeLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::CropAndResizeLayer"]), _)]),
	cv::Ptr<cv::dnn::CropAndResizeLayer>* cv_PtrLcv_dnn_CropAndResizeLayerG_new_const_CropAndResizeLayer(cv::dnn::CropAndResizeLayer* val) {
			return new cv::Ptr<cv::dnn::CropAndResizeLayer>(val);
	}

}

