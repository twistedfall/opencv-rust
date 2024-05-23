extern "C" {
	// cv::Ptr<cv::dnn::ConstantOfShapeLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::ConstantOfShapeLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::ConstantOfShapeLayer* cv_PtrLcv_dnn_ConstantOfShapeLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::ConstantOfShapeLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ConstantOfShapeLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::ConstantOfShapeLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ConstantOfShapeLayer* cv_PtrLcv_dnn_ConstantOfShapeLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::ConstantOfShapeLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ConstantOfShapeLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::ConstantOfShapeLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::ConstantOfShapeLayer>* cv_PtrLcv_dnn_ConstantOfShapeLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::ConstantOfShapeLayer>();
	}

	// cv::Ptr<cv::dnn::ConstantOfShapeLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::ConstantOfShapeLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_ConstantOfShapeLayerG_delete(cv::Ptr<cv::dnn::ConstantOfShapeLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::ConstantOfShapeLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::ConstantOfShapeLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_ConstantOfShapeLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::ConstantOfShapeLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::ConstantOfShapeLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::ConstantOfShapeLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_ConstantOfShapeLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::ConstantOfShapeLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::ConstantOfShapeLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::ConstantOfShapeLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::ConstantOfShapeLayer"]), _)]),
	cv::Ptr<cv::dnn::ConstantOfShapeLayer>* cv_PtrLcv_dnn_ConstantOfShapeLayerG_new_const_ConstantOfShapeLayer(cv::dnn::ConstantOfShapeLayer* val) {
			return new cv::Ptr<cv::dnn::ConstantOfShapeLayer>(val);
	}

}

