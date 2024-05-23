extern "C" {
	// cv::Ptr<cv::dnn::TileLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::TileLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::TileLayer* cv_PtrLcv_dnn_TileLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::TileLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::TileLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::TileLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::TileLayer* cv_PtrLcv_dnn_TileLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::TileLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::TileLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::TileLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::TileLayer>* cv_PtrLcv_dnn_TileLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::TileLayer>();
	}

	// cv::Ptr<cv::dnn::TileLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::TileLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_TileLayerG_delete(cv::Ptr<cv::dnn::TileLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::TileLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::TileLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_TileLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::TileLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::TileLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::TileLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_TileLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::TileLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::TileLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::TileLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::TileLayer"]), _)]),
	cv::Ptr<cv::dnn::TileLayer>* cv_PtrLcv_dnn_TileLayerG_new_const_TileLayer(cv::dnn::TileLayer* val) {
			return new cv::Ptr<cv::dnn::TileLayer>(val);
	}

}

