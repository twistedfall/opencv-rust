extern "C" {
	// cv::Ptr<cv::img_hash::PHash>::getInnerPtr() generated
	// ("cv::Ptr<cv::img_hash::PHash>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::img_hash::PHash* cv_PtrLcv_img_hash_PHashG_getInnerPtr_const(const cv::Ptr<cv::img_hash::PHash>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::img_hash::PHash>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::img_hash::PHash>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::img_hash::PHash* cv_PtrLcv_img_hash_PHashG_getInnerPtrMut(cv::Ptr<cv::img_hash::PHash>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::img_hash::PHash>::new_null() generated
	// ("cv::Ptr<cv::img_hash::PHash>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::img_hash::PHash>* cv_PtrLcv_img_hash_PHashG_new_null_const() {
			return new cv::Ptr<cv::img_hash::PHash>();
	}

	// cv::Ptr<cv::img_hash::PHash>::delete() generated
	// ("cv::Ptr<cv::img_hash::PHash>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_img_hash_PHashG_delete(cv::Ptr<cv::img_hash::PHash>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::img_hash::PHash>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::img_hash::PHash>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_img_hash_PHashG_to_PtrOfAlgorithm(cv::Ptr<cv::img_hash::PHash>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::img_hash::PHash>::to_PtrOfImgHashBase() generated
	// ("cv::Ptr<cv::img_hash::PHash>::to_PtrOfImgHashBase", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::img_hash::ImgHashBase>* cv_PtrLcv_img_hash_PHashG_to_PtrOfImgHashBase(cv::Ptr<cv::img_hash::PHash>* instance) {
			return new cv::Ptr<cv::img_hash::ImgHashBase>(instance->dynamicCast<cv::img_hash::ImgHashBase>());
	}

	// cv::Ptr<cv::img_hash::PHash>::new(TraitClass) generated
	// ("cv::Ptr<cv::img_hash::PHash>::new", vec![(pred!(const, ["val"], ["cv::img_hash::PHash"]), _)]),
	cv::Ptr<cv::img_hash::PHash>* cv_PtrLcv_img_hash_PHashG_new_const_PHash(cv::img_hash::PHash* val) {
			return new cv::Ptr<cv::img_hash::PHash>(val);
	}

}

