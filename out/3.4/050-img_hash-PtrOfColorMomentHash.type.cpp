extern "C" {
	// cv::Ptr<cv::img_hash::ColorMomentHash>::getInnerPtr() generated
	// ("cv::Ptr<cv::img_hash::ColorMomentHash>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::img_hash::ColorMomentHash* cv_PtrLcv_img_hash_ColorMomentHashG_getInnerPtr_const(const cv::Ptr<cv::img_hash::ColorMomentHash>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::img_hash::ColorMomentHash>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::img_hash::ColorMomentHash>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::img_hash::ColorMomentHash* cv_PtrLcv_img_hash_ColorMomentHashG_getInnerPtrMut(cv::Ptr<cv::img_hash::ColorMomentHash>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::img_hash::ColorMomentHash>::new_null() generated
	// ("cv::Ptr<cv::img_hash::ColorMomentHash>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::img_hash::ColorMomentHash>* cv_PtrLcv_img_hash_ColorMomentHashG_new_null_const() {
			return new cv::Ptr<cv::img_hash::ColorMomentHash>();
	}

	// cv::Ptr<cv::img_hash::ColorMomentHash>::delete() generated
	// ("cv::Ptr<cv::img_hash::ColorMomentHash>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_img_hash_ColorMomentHashG_delete(cv::Ptr<cv::img_hash::ColorMomentHash>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::img_hash::ColorMomentHash>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::img_hash::ColorMomentHash>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_img_hash_ColorMomentHashG_to_PtrOfAlgorithm(cv::Ptr<cv::img_hash::ColorMomentHash>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::img_hash::ColorMomentHash>::to_PtrOfImgHashBase() generated
	// ("cv::Ptr<cv::img_hash::ColorMomentHash>::to_PtrOfImgHashBase", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::img_hash::ImgHashBase>* cv_PtrLcv_img_hash_ColorMomentHashG_to_PtrOfImgHashBase(cv::Ptr<cv::img_hash::ColorMomentHash>* instance) {
			return new cv::Ptr<cv::img_hash::ImgHashBase>(instance->dynamicCast<cv::img_hash::ImgHashBase>());
	}

	// cv::Ptr<cv::img_hash::ColorMomentHash>::new(TraitClass) generated
	// ("cv::Ptr<cv::img_hash::ColorMomentHash>::new", vec![(pred!(const, ["val"], ["cv::img_hash::ColorMomentHash"]), _)]),
	cv::Ptr<cv::img_hash::ColorMomentHash>* cv_PtrLcv_img_hash_ColorMomentHashG_new_const_ColorMomentHash(cv::img_hash::ColorMomentHash* val) {
			return new cv::Ptr<cv::img_hash::ColorMomentHash>(val);
	}

}

