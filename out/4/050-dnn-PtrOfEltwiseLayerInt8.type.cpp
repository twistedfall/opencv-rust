extern "C" {
	// cv::Ptr<cv::dnn::EltwiseLayerInt8>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::EltwiseLayerInt8>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::EltwiseLayerInt8* cv_PtrLcv_dnn_EltwiseLayerInt8G_getInnerPtr_const(const cv::Ptr<cv::dnn::EltwiseLayerInt8>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::EltwiseLayerInt8>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::EltwiseLayerInt8>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::EltwiseLayerInt8* cv_PtrLcv_dnn_EltwiseLayerInt8G_getInnerPtrMut(cv::Ptr<cv::dnn::EltwiseLayerInt8>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::EltwiseLayerInt8>::new_null() generated
	// ("cv::Ptr<cv::dnn::EltwiseLayerInt8>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::EltwiseLayerInt8>* cv_PtrLcv_dnn_EltwiseLayerInt8G_new_null_const() {
			return new cv::Ptr<cv::dnn::EltwiseLayerInt8>();
	}

	// cv::Ptr<cv::dnn::EltwiseLayerInt8>::delete() generated
	// ("cv::Ptr<cv::dnn::EltwiseLayerInt8>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_EltwiseLayerInt8G_delete(cv::Ptr<cv::dnn::EltwiseLayerInt8>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::EltwiseLayerInt8>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::EltwiseLayerInt8>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_EltwiseLayerInt8G_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::EltwiseLayerInt8>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::EltwiseLayerInt8>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::EltwiseLayerInt8>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_EltwiseLayerInt8G_to_PtrOfLayer(cv::Ptr<cv::dnn::EltwiseLayerInt8>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::EltwiseLayerInt8>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::EltwiseLayerInt8>::new", vec![(pred!(const, ["val"], ["cv::dnn::EltwiseLayerInt8"]), _)]),
	cv::Ptr<cv::dnn::EltwiseLayerInt8>* cv_PtrLcv_dnn_EltwiseLayerInt8G_new_const_EltwiseLayerInt8(cv::dnn::EltwiseLayerInt8* val) {
			return new cv::Ptr<cv::dnn::EltwiseLayerInt8>(val);
	}

}

