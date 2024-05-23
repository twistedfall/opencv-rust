extern "C" {
	// cv::Ptr<cv::img_hash::ImgHashBase>::getInnerPtr() generated
	// ("cv::Ptr<cv::img_hash::ImgHashBase>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::img_hash::ImgHashBase* cv_PtrLcv_img_hash_ImgHashBaseG_getInnerPtr_const(const cv::Ptr<cv::img_hash::ImgHashBase>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::img_hash::ImgHashBase>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::img_hash::ImgHashBase>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::img_hash::ImgHashBase* cv_PtrLcv_img_hash_ImgHashBaseG_getInnerPtrMut(cv::Ptr<cv::img_hash::ImgHashBase>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::img_hash::ImgHashBase>::new_null() generated
	// ("cv::Ptr<cv::img_hash::ImgHashBase>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::img_hash::ImgHashBase>* cv_PtrLcv_img_hash_ImgHashBaseG_new_null_const() {
			return new cv::Ptr<cv::img_hash::ImgHashBase>();
	}

	// cv::Ptr<cv::img_hash::ImgHashBase>::delete() generated
	// ("cv::Ptr<cv::img_hash::ImgHashBase>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_img_hash_ImgHashBaseG_delete(cv::Ptr<cv::img_hash::ImgHashBase>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::img_hash::ImgHashBase>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::img_hash::ImgHashBase>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_img_hash_ImgHashBaseG_to_PtrOfAlgorithm(cv::Ptr<cv::img_hash::ImgHashBase>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::img_hash::ImgHashBase>::new(TraitClass) generated
	// ("cv::Ptr<cv::img_hash::ImgHashBase>::new", vec![(pred!(const, ["val"], ["cv::img_hash::ImgHashBase"]), _)]),
	cv::Ptr<cv::img_hash::ImgHashBase>* cv_PtrLcv_img_hash_ImgHashBaseG_new_const_ImgHashBase(cv::img_hash::ImgHashBase* val) {
			return new cv::Ptr<cv::img_hash::ImgHashBase>(val);
	}

}

