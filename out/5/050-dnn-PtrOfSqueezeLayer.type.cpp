extern "C" {
	// cv::Ptr<cv::dnn::SqueezeLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::SqueezeLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::SqueezeLayer* cv_PtrLcv_dnn_SqueezeLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::SqueezeLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::SqueezeLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::SqueezeLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::SqueezeLayer* cv_PtrLcv_dnn_SqueezeLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::SqueezeLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::SqueezeLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::SqueezeLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::SqueezeLayer>* cv_PtrLcv_dnn_SqueezeLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::SqueezeLayer>();
	}

	// cv::Ptr<cv::dnn::SqueezeLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::SqueezeLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_SqueezeLayerG_delete(cv::Ptr<cv::dnn::SqueezeLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::SqueezeLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::SqueezeLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_SqueezeLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::SqueezeLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::SqueezeLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::SqueezeLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_SqueezeLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::SqueezeLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::SqueezeLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::SqueezeLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::SqueezeLayer"]), _)]),
	cv::Ptr<cv::dnn::SqueezeLayer>* cv_PtrLcv_dnn_SqueezeLayerG_new_const_SqueezeLayer(cv::dnn::SqueezeLayer* val) {
			return new cv::Ptr<cv::dnn::SqueezeLayer>(val);
	}

}

