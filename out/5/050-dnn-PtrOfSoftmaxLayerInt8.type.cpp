extern "C" {
	// cv::Ptr<cv::dnn::SoftmaxLayerInt8>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::SoftmaxLayerInt8>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::SoftmaxLayerInt8* cv_PtrLcv_dnn_SoftmaxLayerInt8G_getInnerPtr_const(const cv::Ptr<cv::dnn::SoftmaxLayerInt8>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::SoftmaxLayerInt8>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::SoftmaxLayerInt8>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::SoftmaxLayerInt8* cv_PtrLcv_dnn_SoftmaxLayerInt8G_getInnerPtrMut(cv::Ptr<cv::dnn::SoftmaxLayerInt8>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::SoftmaxLayerInt8>::new_null() generated
	// ("cv::Ptr<cv::dnn::SoftmaxLayerInt8>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::SoftmaxLayerInt8>* cv_PtrLcv_dnn_SoftmaxLayerInt8G_new_null_const() {
			return new cv::Ptr<cv::dnn::SoftmaxLayerInt8>();
	}

	// cv::Ptr<cv::dnn::SoftmaxLayerInt8>::delete() generated
	// ("cv::Ptr<cv::dnn::SoftmaxLayerInt8>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_SoftmaxLayerInt8G_delete(cv::Ptr<cv::dnn::SoftmaxLayerInt8>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::SoftmaxLayerInt8>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::SoftmaxLayerInt8>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_SoftmaxLayerInt8G_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::SoftmaxLayerInt8>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::SoftmaxLayerInt8>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::SoftmaxLayerInt8>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_SoftmaxLayerInt8G_to_PtrOfLayer(cv::Ptr<cv::dnn::SoftmaxLayerInt8>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::SoftmaxLayerInt8>::to_PtrOfSoftmaxLayer() generated
	// ("cv::Ptr<cv::dnn::SoftmaxLayerInt8>::to_PtrOfSoftmaxLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::SoftmaxLayer>* cv_PtrLcv_dnn_SoftmaxLayerInt8G_to_PtrOfSoftmaxLayer(cv::Ptr<cv::dnn::SoftmaxLayerInt8>* instance) {
			return new cv::Ptr<cv::dnn::SoftmaxLayer>(instance->dynamicCast<cv::dnn::SoftmaxLayer>());
	}

	// cv::Ptr<cv::dnn::SoftmaxLayerInt8>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::SoftmaxLayerInt8>::new", vec![(pred!(const, ["val"], ["cv::dnn::SoftmaxLayerInt8"]), _)]),
	cv::Ptr<cv::dnn::SoftmaxLayerInt8>* cv_PtrLcv_dnn_SoftmaxLayerInt8G_new_const_SoftmaxLayerInt8(cv::dnn::SoftmaxLayerInt8* val) {
			return new cv::Ptr<cv::dnn::SoftmaxLayerInt8>(val);
	}

}

