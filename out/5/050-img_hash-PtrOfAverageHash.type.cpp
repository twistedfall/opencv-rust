extern "C" {
	// cv::Ptr<cv::img_hash::AverageHash>::getInnerPtr() generated
	// ("cv::Ptr<cv::img_hash::AverageHash>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::img_hash::AverageHash* cv_PtrLcv_img_hash_AverageHashG_getInnerPtr_const(const cv::Ptr<cv::img_hash::AverageHash>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::img_hash::AverageHash>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::img_hash::AverageHash>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::img_hash::AverageHash* cv_PtrLcv_img_hash_AverageHashG_getInnerPtrMut(cv::Ptr<cv::img_hash::AverageHash>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::img_hash::AverageHash>::new_null() generated
	// ("cv::Ptr<cv::img_hash::AverageHash>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::img_hash::AverageHash>* cv_PtrLcv_img_hash_AverageHashG_new_null_const() {
			return new cv::Ptr<cv::img_hash::AverageHash>();
	}

	// cv::Ptr<cv::img_hash::AverageHash>::delete() generated
	// ("cv::Ptr<cv::img_hash::AverageHash>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_img_hash_AverageHashG_delete(cv::Ptr<cv::img_hash::AverageHash>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::img_hash::AverageHash>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::img_hash::AverageHash>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_img_hash_AverageHashG_to_PtrOfAlgorithm(cv::Ptr<cv::img_hash::AverageHash>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::img_hash::AverageHash>::to_PtrOfImgHashBase() generated
	// ("cv::Ptr<cv::img_hash::AverageHash>::to_PtrOfImgHashBase", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::img_hash::ImgHashBase>* cv_PtrLcv_img_hash_AverageHashG_to_PtrOfImgHashBase(cv::Ptr<cv::img_hash::AverageHash>* instance) {
			return new cv::Ptr<cv::img_hash::ImgHashBase>(instance->dynamicCast<cv::img_hash::ImgHashBase>());
	}

	// cv::Ptr<cv::img_hash::AverageHash>::new(TraitClass) generated
	// ("cv::Ptr<cv::img_hash::AverageHash>::new", vec![(pred!(const, ["val"], ["cv::img_hash::AverageHash"]), _)]),
	cv::Ptr<cv::img_hash::AverageHash>* cv_PtrLcv_img_hash_AverageHashG_new_const_AverageHash(cv::img_hash::AverageHash* val) {
			return new cv::Ptr<cv::img_hash::AverageHash>(val);
	}

}

