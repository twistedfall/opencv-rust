extern "C" {
	// cv::Ptr<cv::dnn::ReLU6Layer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::ReLU6Layer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::ReLU6Layer* cv_PtrLcv_dnn_ReLU6LayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::ReLU6Layer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ReLU6Layer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::ReLU6Layer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ReLU6Layer* cv_PtrLcv_dnn_ReLU6LayerG_getInnerPtrMut(cv::Ptr<cv::dnn::ReLU6Layer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ReLU6Layer>::new_null() generated
	// ("cv::Ptr<cv::dnn::ReLU6Layer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::ReLU6Layer>* cv_PtrLcv_dnn_ReLU6LayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::ReLU6Layer>();
	}

	// cv::Ptr<cv::dnn::ReLU6Layer>::delete() generated
	// ("cv::Ptr<cv::dnn::ReLU6Layer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_ReLU6LayerG_delete(cv::Ptr<cv::dnn::ReLU6Layer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::ReLU6Layer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::ReLU6Layer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_ReLU6LayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::ReLU6Layer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::ReLU6Layer>::to_PtrOfActivationLayer() generated
	// ("cv::Ptr<cv::dnn::ReLU6Layer>::to_PtrOfActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::ActivationLayer>* cv_PtrLcv_dnn_ReLU6LayerG_to_PtrOfActivationLayer(cv::Ptr<cv::dnn::ReLU6Layer>* instance) {
			return new cv::Ptr<cv::dnn::ActivationLayer>(instance->dynamicCast<cv::dnn::ActivationLayer>());
	}

	// cv::Ptr<cv::dnn::ReLU6Layer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::ReLU6Layer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_ReLU6LayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::ReLU6Layer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::ReLU6Layer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::ReLU6Layer>::new", vec![(pred!(const, ["val"], ["cv::dnn::ReLU6Layer"]), _)]),
	cv::Ptr<cv::dnn::ReLU6Layer>* cv_PtrLcv_dnn_ReLU6LayerG_new_const_ReLU6Layer(cv::dnn::ReLU6Layer* val) {
			return new cv::Ptr<cv::dnn::ReLU6Layer>(val);
	}

}

