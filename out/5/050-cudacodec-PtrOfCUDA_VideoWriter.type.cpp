extern "C" {
	// cv::Ptr<cv::cudacodec::VideoWriter>::getInnerPtr() generated
	// ("cv::Ptr<cv::cudacodec::VideoWriter>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::cudacodec::VideoWriter* cv_PtrLcv_cudacodec_VideoWriterG_getInnerPtr_const(const cv::Ptr<cv::cudacodec::VideoWriter>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cudacodec::VideoWriter>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::cudacodec::VideoWriter>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::cudacodec::VideoWriter* cv_PtrLcv_cudacodec_VideoWriterG_getInnerPtrMut(cv::Ptr<cv::cudacodec::VideoWriter>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cudacodec::VideoWriter>::new_null() generated
	// ("cv::Ptr<cv::cudacodec::VideoWriter>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::cudacodec::VideoWriter>* cv_PtrLcv_cudacodec_VideoWriterG_new_null_const() {
			return new cv::Ptr<cv::cudacodec::VideoWriter>();
	}

	// cv::Ptr<cv::cudacodec::VideoWriter>::delete() generated
	// ("cv::Ptr<cv::cudacodec::VideoWriter>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_cudacodec_VideoWriterG_delete(cv::Ptr<cv::cudacodec::VideoWriter>* instance) {
			delete instance;
	}

}

