extern "C" {
	// cv::Ptr<cv::dnn::UnsqueezeLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::UnsqueezeLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::UnsqueezeLayer* cv_PtrLcv_dnn_UnsqueezeLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::UnsqueezeLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::UnsqueezeLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::UnsqueezeLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::UnsqueezeLayer* cv_PtrLcv_dnn_UnsqueezeLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::UnsqueezeLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::UnsqueezeLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::UnsqueezeLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::UnsqueezeLayer>* cv_PtrLcv_dnn_UnsqueezeLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::UnsqueezeLayer>();
	}

	// cv::Ptr<cv::dnn::UnsqueezeLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::UnsqueezeLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_UnsqueezeLayerG_delete(cv::Ptr<cv::dnn::UnsqueezeLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::UnsqueezeLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::UnsqueezeLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_UnsqueezeLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::UnsqueezeLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::UnsqueezeLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::UnsqueezeLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_UnsqueezeLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::UnsqueezeLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::UnsqueezeLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::UnsqueezeLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::UnsqueezeLayer"]), _)]),
	cv::Ptr<cv::dnn::UnsqueezeLayer>* cv_PtrLcv_dnn_UnsqueezeLayerG_new_const_UnsqueezeLayer(cv::dnn::UnsqueezeLayer* val) {
			return new cv::Ptr<cv::dnn::UnsqueezeLayer>(val);
	}

}

