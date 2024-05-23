extern "C" {
	// cv::Ptr<cv::dnn::Layer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::Layer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::Layer* cv_PtrLcv_dnn_LayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::Layer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::Layer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::Layer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_PtrLcv_dnn_LayerG_getInnerPtrMut(cv::Ptr<cv::dnn::Layer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::Layer>::new_null() generated
	// ("cv::Ptr<cv::dnn::Layer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_LayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::Layer>();
	}

	// cv::Ptr<cv::dnn::Layer>::delete() generated
	// ("cv::Ptr<cv::dnn::Layer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_LayerG_delete(cv::Ptr<cv::dnn::Layer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::Layer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::Layer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_LayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::Layer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::Layer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::Layer>::new", vec![(pred!(const, ["val"], ["cv::dnn::Layer"]), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_LayerG_new_const_Layer(cv::dnn::Layer* val) {
			return new cv::Ptr<cv::dnn::Layer>(val);
	}

}

