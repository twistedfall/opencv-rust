extern "C" {
	// cv::Ptr<cv::dnn::InnerProductLayerInt8>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::InnerProductLayerInt8>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::InnerProductLayerInt8* cv_PtrLcv_dnn_InnerProductLayerInt8G_getInnerPtr_const(const cv::Ptr<cv::dnn::InnerProductLayerInt8>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::InnerProductLayerInt8>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::InnerProductLayerInt8>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::InnerProductLayerInt8* cv_PtrLcv_dnn_InnerProductLayerInt8G_getInnerPtrMut(cv::Ptr<cv::dnn::InnerProductLayerInt8>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::InnerProductLayerInt8>::new_null() generated
	// ("cv::Ptr<cv::dnn::InnerProductLayerInt8>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::InnerProductLayerInt8>* cv_PtrLcv_dnn_InnerProductLayerInt8G_new_null_const() {
			return new cv::Ptr<cv::dnn::InnerProductLayerInt8>();
	}

	// cv::Ptr<cv::dnn::InnerProductLayerInt8>::delete() generated
	// ("cv::Ptr<cv::dnn::InnerProductLayerInt8>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_InnerProductLayerInt8G_delete(cv::Ptr<cv::dnn::InnerProductLayerInt8>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::InnerProductLayerInt8>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::InnerProductLayerInt8>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_InnerProductLayerInt8G_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::InnerProductLayerInt8>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::InnerProductLayerInt8>::to_PtrOfInnerProductLayer() generated
	// ("cv::Ptr<cv::dnn::InnerProductLayerInt8>::to_PtrOfInnerProductLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::InnerProductLayer>* cv_PtrLcv_dnn_InnerProductLayerInt8G_to_PtrOfInnerProductLayer(cv::Ptr<cv::dnn::InnerProductLayerInt8>* instance) {
			return new cv::Ptr<cv::dnn::InnerProductLayer>(instance->dynamicCast<cv::dnn::InnerProductLayer>());
	}

	// cv::Ptr<cv::dnn::InnerProductLayerInt8>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::InnerProductLayerInt8>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_InnerProductLayerInt8G_to_PtrOfLayer(cv::Ptr<cv::dnn::InnerProductLayerInt8>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::InnerProductLayerInt8>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::InnerProductLayerInt8>::new", vec![(pred!(const, ["val"], ["cv::dnn::InnerProductLayerInt8"]), _)]),
	cv::Ptr<cv::dnn::InnerProductLayerInt8>* cv_PtrLcv_dnn_InnerProductLayerInt8G_new_const_InnerProductLayerInt8(cv::dnn::InnerProductLayerInt8* val) {
			return new cv::Ptr<cv::dnn::InnerProductLayerInt8>(val);
	}

}

