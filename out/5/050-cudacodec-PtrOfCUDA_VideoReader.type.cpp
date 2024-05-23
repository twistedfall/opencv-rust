extern "C" {
	// cv::Ptr<cv::cudacodec::VideoReader>::getInnerPtr() generated
	// ("cv::Ptr<cv::cudacodec::VideoReader>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::cudacodec::VideoReader* cv_PtrLcv_cudacodec_VideoReaderG_getInnerPtr_const(const cv::Ptr<cv::cudacodec::VideoReader>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cudacodec::VideoReader>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::cudacodec::VideoReader>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::cudacodec::VideoReader* cv_PtrLcv_cudacodec_VideoReaderG_getInnerPtrMut(cv::Ptr<cv::cudacodec::VideoReader>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cudacodec::VideoReader>::new_null() generated
	// ("cv::Ptr<cv::cudacodec::VideoReader>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::cudacodec::VideoReader>* cv_PtrLcv_cudacodec_VideoReaderG_new_null_const() {
			return new cv::Ptr<cv::cudacodec::VideoReader>();
	}

	// cv::Ptr<cv::cudacodec::VideoReader>::delete() generated
	// ("cv::Ptr<cv::cudacodec::VideoReader>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_cudacodec_VideoReaderG_delete(cv::Ptr<cv::cudacodec::VideoReader>* instance) {
			delete instance;
	}

}

