extern "C" {
	// cv::Ptr<cv::dnn::BackendNode>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::BackendNode>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::BackendNode* cv_PtrLcv_dnn_BackendNodeG_getInnerPtr_const(const cv::Ptr<cv::dnn::BackendNode>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::BackendNode>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::BackendNode>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::BackendNode* cv_PtrLcv_dnn_BackendNodeG_getInnerPtrMut(cv::Ptr<cv::dnn::BackendNode>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::BackendNode>::new_null() generated
	// ("cv::Ptr<cv::dnn::BackendNode>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::BackendNode>* cv_PtrLcv_dnn_BackendNodeG_new_null_const() {
			return new cv::Ptr<cv::dnn::BackendNode>();
	}

	// cv::Ptr<cv::dnn::BackendNode>::delete() generated
	// ("cv::Ptr<cv::dnn::BackendNode>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_BackendNodeG_delete(cv::Ptr<cv::dnn::BackendNode>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::BackendNode>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::BackendNode>::new", vec![(pred!(const, ["val"], ["cv::dnn::BackendNode"]), _)]),
	cv::Ptr<cv::dnn::BackendNode>* cv_PtrLcv_dnn_BackendNodeG_new_const_BackendNode(cv::dnn::BackendNode* val) {
			return new cv::Ptr<cv::dnn::BackendNode>(val);
	}

}

