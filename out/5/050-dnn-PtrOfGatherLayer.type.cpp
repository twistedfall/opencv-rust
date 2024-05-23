extern "C" {
	// cv::Ptr<cv::dnn::GatherLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::GatherLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::GatherLayer* cv_PtrLcv_dnn_GatherLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::GatherLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::GatherLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::GatherLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::GatherLayer* cv_PtrLcv_dnn_GatherLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::GatherLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::GatherLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::GatherLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::GatherLayer>* cv_PtrLcv_dnn_GatherLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::GatherLayer>();
	}

	// cv::Ptr<cv::dnn::GatherLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::GatherLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_GatherLayerG_delete(cv::Ptr<cv::dnn::GatherLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::GatherLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::GatherLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_GatherLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::GatherLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::GatherLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::GatherLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_GatherLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::GatherLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::GatherLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::GatherLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::GatherLayer"]), _)]),
	cv::Ptr<cv::dnn::GatherLayer>* cv_PtrLcv_dnn_GatherLayerG_new_const_GatherLayer(cv::dnn::GatherLayer* val) {
			return new cv::Ptr<cv::dnn::GatherLayer>(val);
	}

}

