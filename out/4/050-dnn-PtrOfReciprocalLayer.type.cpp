extern "C" {
	// cv::Ptr<cv::dnn::ReciprocalLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::ReciprocalLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::ReciprocalLayer* cv_PtrLcv_dnn_ReciprocalLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::ReciprocalLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ReciprocalLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::ReciprocalLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ReciprocalLayer* cv_PtrLcv_dnn_ReciprocalLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::ReciprocalLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ReciprocalLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::ReciprocalLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::ReciprocalLayer>* cv_PtrLcv_dnn_ReciprocalLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::ReciprocalLayer>();
	}

	// cv::Ptr<cv::dnn::ReciprocalLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::ReciprocalLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_ReciprocalLayerG_delete(cv::Ptr<cv::dnn::ReciprocalLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::ReciprocalLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::ReciprocalLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_ReciprocalLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::ReciprocalLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::ReciprocalLayer>::to_PtrOfActivationLayer() generated
	// ("cv::Ptr<cv::dnn::ReciprocalLayer>::to_PtrOfActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::ActivationLayer>* cv_PtrLcv_dnn_ReciprocalLayerG_to_PtrOfActivationLayer(cv::Ptr<cv::dnn::ReciprocalLayer>* instance) {
			return new cv::Ptr<cv::dnn::ActivationLayer>(instance->dynamicCast<cv::dnn::ActivationLayer>());
	}

	// cv::Ptr<cv::dnn::ReciprocalLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::ReciprocalLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_ReciprocalLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::ReciprocalLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::ReciprocalLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::ReciprocalLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::ReciprocalLayer"]), _)]),
	cv::Ptr<cv::dnn::ReciprocalLayer>* cv_PtrLcv_dnn_ReciprocalLayerG_new_const_ReciprocalLayer(cv::dnn::ReciprocalLayer* val) {
			return new cv::Ptr<cv::dnn::ReciprocalLayer>(val);
	}

}

