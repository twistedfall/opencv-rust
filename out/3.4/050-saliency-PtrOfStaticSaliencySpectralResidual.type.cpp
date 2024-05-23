extern "C" {
	// cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>::getInnerPtr() generated
	// ("cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::saliency::StaticSaliencySpectralResidual* cv_PtrLcv_saliency_StaticSaliencySpectralResidualG_getInnerPtr_const(const cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::saliency::StaticSaliencySpectralResidual* cv_PtrLcv_saliency_StaticSaliencySpectralResidualG_getInnerPtrMut(cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>::new_null() generated
	// ("cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>* cv_PtrLcv_saliency_StaticSaliencySpectralResidualG_new_null_const() {
			return new cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>();
	}

	// cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>::delete() generated
	// ("cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_saliency_StaticSaliencySpectralResidualG_delete(cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_saliency_StaticSaliencySpectralResidualG_to_PtrOfAlgorithm(cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>::to_PtrOfSaliency() generated
	// ("cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>::to_PtrOfSaliency", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::saliency::Saliency>* cv_PtrLcv_saliency_StaticSaliencySpectralResidualG_to_PtrOfSaliency(cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>* instance) {
			return new cv::Ptr<cv::saliency::Saliency>(instance->dynamicCast<cv::saliency::Saliency>());
	}

	// cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>::to_PtrOfStaticSaliency() generated
	// ("cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>::to_PtrOfStaticSaliency", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::saliency::StaticSaliency>* cv_PtrLcv_saliency_StaticSaliencySpectralResidualG_to_PtrOfStaticSaliency(cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>* instance) {
			return new cv::Ptr<cv::saliency::StaticSaliency>(instance->dynamicCast<cv::saliency::StaticSaliency>());
	}

	// cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>::new(TraitClass) generated
	// ("cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>::new", vec![(pred!(const, ["val"], ["cv::saliency::StaticSaliencySpectralResidual"]), _)]),
	cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>* cv_PtrLcv_saliency_StaticSaliencySpectralResidualG_new_const_StaticSaliencySpectralResidual(cv::saliency::StaticSaliencySpectralResidual* val) {
			return new cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>(val);
	}

}

