extern "C" {
	// cv::Ptr<cv::img_hash::MarrHildrethHash>::getInnerPtr() generated
	// ("cv::Ptr<cv::img_hash::MarrHildrethHash>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::img_hash::MarrHildrethHash* cv_PtrLcv_img_hash_MarrHildrethHashG_getInnerPtr_const(const cv::Ptr<cv::img_hash::MarrHildrethHash>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::img_hash::MarrHildrethHash>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::img_hash::MarrHildrethHash>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::img_hash::MarrHildrethHash* cv_PtrLcv_img_hash_MarrHildrethHashG_getInnerPtrMut(cv::Ptr<cv::img_hash::MarrHildrethHash>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::img_hash::MarrHildrethHash>::new_null() generated
	// ("cv::Ptr<cv::img_hash::MarrHildrethHash>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::img_hash::MarrHildrethHash>* cv_PtrLcv_img_hash_MarrHildrethHashG_new_null_const() {
			return new cv::Ptr<cv::img_hash::MarrHildrethHash>();
	}

	// cv::Ptr<cv::img_hash::MarrHildrethHash>::delete() generated
	// ("cv::Ptr<cv::img_hash::MarrHildrethHash>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_img_hash_MarrHildrethHashG_delete(cv::Ptr<cv::img_hash::MarrHildrethHash>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::img_hash::MarrHildrethHash>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::img_hash::MarrHildrethHash>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_img_hash_MarrHildrethHashG_to_PtrOfAlgorithm(cv::Ptr<cv::img_hash::MarrHildrethHash>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::img_hash::MarrHildrethHash>::to_PtrOfImgHashBase() generated
	// ("cv::Ptr<cv::img_hash::MarrHildrethHash>::to_PtrOfImgHashBase", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::img_hash::ImgHashBase>* cv_PtrLcv_img_hash_MarrHildrethHashG_to_PtrOfImgHashBase(cv::Ptr<cv::img_hash::MarrHildrethHash>* instance) {
			return new cv::Ptr<cv::img_hash::ImgHashBase>(instance->dynamicCast<cv::img_hash::ImgHashBase>());
	}

	// cv::Ptr<cv::img_hash::MarrHildrethHash>::new(TraitClass) generated
	// ("cv::Ptr<cv::img_hash::MarrHildrethHash>::new", vec![(pred!(const, ["val"], ["cv::img_hash::MarrHildrethHash"]), _)]),
	cv::Ptr<cv::img_hash::MarrHildrethHash>* cv_PtrLcv_img_hash_MarrHildrethHashG_new_const_MarrHildrethHash(cv::img_hash::MarrHildrethHash* val) {
			return new cv::Ptr<cv::img_hash::MarrHildrethHash>(val);
	}

}

