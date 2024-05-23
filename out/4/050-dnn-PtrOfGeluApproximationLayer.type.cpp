extern "C" {
	// cv::Ptr<cv::dnn::GeluApproximationLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::GeluApproximationLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::GeluApproximationLayer* cv_PtrLcv_dnn_GeluApproximationLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::GeluApproximationLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::GeluApproximationLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::GeluApproximationLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::GeluApproximationLayer* cv_PtrLcv_dnn_GeluApproximationLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::GeluApproximationLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::GeluApproximationLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::GeluApproximationLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::GeluApproximationLayer>* cv_PtrLcv_dnn_GeluApproximationLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::GeluApproximationLayer>();
	}

	// cv::Ptr<cv::dnn::GeluApproximationLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::GeluApproximationLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_GeluApproximationLayerG_delete(cv::Ptr<cv::dnn::GeluApproximationLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::GeluApproximationLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::GeluApproximationLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_GeluApproximationLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::GeluApproximationLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::GeluApproximationLayer>::to_PtrOfActivationLayer() generated
	// ("cv::Ptr<cv::dnn::GeluApproximationLayer>::to_PtrOfActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::ActivationLayer>* cv_PtrLcv_dnn_GeluApproximationLayerG_to_PtrOfActivationLayer(cv::Ptr<cv::dnn::GeluApproximationLayer>* instance) {
			return new cv::Ptr<cv::dnn::ActivationLayer>(instance->dynamicCast<cv::dnn::ActivationLayer>());
	}

	// cv::Ptr<cv::dnn::GeluApproximationLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::GeluApproximationLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_GeluApproximationLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::GeluApproximationLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::GeluApproximationLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::GeluApproximationLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::GeluApproximationLayer"]), _)]),
	cv::Ptr<cv::dnn::GeluApproximationLayer>* cv_PtrLcv_dnn_GeluApproximationLayerG_new_const_GeluApproximationLayer(cv::dnn::GeluApproximationLayer* val) {
			return new cv::Ptr<cv::dnn::GeluApproximationLayer>(val);
	}

}

