extern "C" {
	// cv::Ptr<cv::dnn::Concat2Layer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::Concat2Layer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::Concat2Layer* cv_PtrLcv_dnn_Concat2LayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::Concat2Layer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::Concat2Layer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::Concat2Layer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Concat2Layer* cv_PtrLcv_dnn_Concat2LayerG_getInnerPtrMut(cv::Ptr<cv::dnn::Concat2Layer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::Concat2Layer>::new_null() generated
	// ("cv::Ptr<cv::dnn::Concat2Layer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::Concat2Layer>* cv_PtrLcv_dnn_Concat2LayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::Concat2Layer>();
	}

	// cv::Ptr<cv::dnn::Concat2Layer>::delete() generated
	// ("cv::Ptr<cv::dnn::Concat2Layer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_Concat2LayerG_delete(cv::Ptr<cv::dnn::Concat2Layer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::Concat2Layer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::Concat2Layer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_Concat2LayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::Concat2Layer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::Concat2Layer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::Concat2Layer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_Concat2LayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::Concat2Layer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::Concat2Layer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::Concat2Layer>::new", vec![(pred!(const, ["val"], ["cv::dnn::Concat2Layer"]), _)]),
	cv::Ptr<cv::dnn::Concat2Layer>* cv_PtrLcv_dnn_Concat2LayerG_new_const_Concat2Layer(cv::dnn::Concat2Layer* val) {
			return new cv::Ptr<cv::dnn::Concat2Layer>(val);
	}

}

