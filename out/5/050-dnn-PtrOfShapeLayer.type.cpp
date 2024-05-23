extern "C" {
	// cv::Ptr<cv::dnn::ShapeLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::ShapeLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::ShapeLayer* cv_PtrLcv_dnn_ShapeLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::ShapeLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ShapeLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::ShapeLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ShapeLayer* cv_PtrLcv_dnn_ShapeLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::ShapeLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ShapeLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::ShapeLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::ShapeLayer>* cv_PtrLcv_dnn_ShapeLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::ShapeLayer>();
	}

	// cv::Ptr<cv::dnn::ShapeLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::ShapeLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_ShapeLayerG_delete(cv::Ptr<cv::dnn::ShapeLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::ShapeLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::ShapeLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_ShapeLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::ShapeLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::ShapeLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::ShapeLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_ShapeLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::ShapeLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::ShapeLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::ShapeLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::ShapeLayer"]), _)]),
	cv::Ptr<cv::dnn::ShapeLayer>* cv_PtrLcv_dnn_ShapeLayerG_new_const_ShapeLayer(cv::dnn::ShapeLayer* val) {
			return new cv::Ptr<cv::dnn::ShapeLayer>(val);
	}

}

