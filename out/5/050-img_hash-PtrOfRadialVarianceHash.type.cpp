extern "C" {
	// cv::Ptr<cv::img_hash::RadialVarianceHash>::getInnerPtr() generated
	// ("cv::Ptr<cv::img_hash::RadialVarianceHash>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::img_hash::RadialVarianceHash* cv_PtrLcv_img_hash_RadialVarianceHashG_getInnerPtr_const(const cv::Ptr<cv::img_hash::RadialVarianceHash>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::img_hash::RadialVarianceHash>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::img_hash::RadialVarianceHash>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::img_hash::RadialVarianceHash* cv_PtrLcv_img_hash_RadialVarianceHashG_getInnerPtrMut(cv::Ptr<cv::img_hash::RadialVarianceHash>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::img_hash::RadialVarianceHash>::new_null() generated
	// ("cv::Ptr<cv::img_hash::RadialVarianceHash>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::img_hash::RadialVarianceHash>* cv_PtrLcv_img_hash_RadialVarianceHashG_new_null_const() {
			return new cv::Ptr<cv::img_hash::RadialVarianceHash>();
	}

	// cv::Ptr<cv::img_hash::RadialVarianceHash>::delete() generated
	// ("cv::Ptr<cv::img_hash::RadialVarianceHash>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_img_hash_RadialVarianceHashG_delete(cv::Ptr<cv::img_hash::RadialVarianceHash>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::img_hash::RadialVarianceHash>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::img_hash::RadialVarianceHash>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_img_hash_RadialVarianceHashG_to_PtrOfAlgorithm(cv::Ptr<cv::img_hash::RadialVarianceHash>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::img_hash::RadialVarianceHash>::to_PtrOfImgHashBase() generated
	// ("cv::Ptr<cv::img_hash::RadialVarianceHash>::to_PtrOfImgHashBase", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::img_hash::ImgHashBase>* cv_PtrLcv_img_hash_RadialVarianceHashG_to_PtrOfImgHashBase(cv::Ptr<cv::img_hash::RadialVarianceHash>* instance) {
			return new cv::Ptr<cv::img_hash::ImgHashBase>(instance->dynamicCast<cv::img_hash::ImgHashBase>());
	}

	// cv::Ptr<cv::img_hash::RadialVarianceHash>::new(TraitClass) generated
	// ("cv::Ptr<cv::img_hash::RadialVarianceHash>::new", vec![(pred!(const, ["val"], ["cv::img_hash::RadialVarianceHash"]), _)]),
	cv::Ptr<cv::img_hash::RadialVarianceHash>* cv_PtrLcv_img_hash_RadialVarianceHashG_new_const_RadialVarianceHash(cv::img_hash::RadialVarianceHash* val) {
			return new cv::Ptr<cv::img_hash::RadialVarianceHash>(val);
	}

}

