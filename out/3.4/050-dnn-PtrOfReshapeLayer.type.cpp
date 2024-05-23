extern "C" {
	// cv::Ptr<cv::dnn::ReshapeLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::ReshapeLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::ReshapeLayer* cv_PtrLcv_dnn_ReshapeLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::ReshapeLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ReshapeLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::ReshapeLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ReshapeLayer* cv_PtrLcv_dnn_ReshapeLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::ReshapeLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ReshapeLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::ReshapeLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::ReshapeLayer>* cv_PtrLcv_dnn_ReshapeLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::ReshapeLayer>();
	}

	// cv::Ptr<cv::dnn::ReshapeLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::ReshapeLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_ReshapeLayerG_delete(cv::Ptr<cv::dnn::ReshapeLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::ReshapeLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::ReshapeLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_ReshapeLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::ReshapeLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::ReshapeLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::ReshapeLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_ReshapeLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::ReshapeLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::ReshapeLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::ReshapeLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::ReshapeLayer"]), _)]),
	cv::Ptr<cv::dnn::ReshapeLayer>* cv_PtrLcv_dnn_ReshapeLayerG_new_const_ReshapeLayer(cv::dnn::ReshapeLayer* val) {
			return new cv::Ptr<cv::dnn::ReshapeLayer>(val);
	}

}

