extern "C" {
	// cv::Ptr<cv::img_hash::BlockMeanHash>::getInnerPtr() generated
	// ("cv::Ptr<cv::img_hash::BlockMeanHash>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::img_hash::BlockMeanHash* cv_PtrLcv_img_hash_BlockMeanHashG_getInnerPtr_const(const cv::Ptr<cv::img_hash::BlockMeanHash>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::img_hash::BlockMeanHash>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::img_hash::BlockMeanHash>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::img_hash::BlockMeanHash* cv_PtrLcv_img_hash_BlockMeanHashG_getInnerPtrMut(cv::Ptr<cv::img_hash::BlockMeanHash>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::img_hash::BlockMeanHash>::new_null() generated
	// ("cv::Ptr<cv::img_hash::BlockMeanHash>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::img_hash::BlockMeanHash>* cv_PtrLcv_img_hash_BlockMeanHashG_new_null_const() {
			return new cv::Ptr<cv::img_hash::BlockMeanHash>();
	}

	// cv::Ptr<cv::img_hash::BlockMeanHash>::delete() generated
	// ("cv::Ptr<cv::img_hash::BlockMeanHash>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_img_hash_BlockMeanHashG_delete(cv::Ptr<cv::img_hash::BlockMeanHash>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::img_hash::BlockMeanHash>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::img_hash::BlockMeanHash>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_img_hash_BlockMeanHashG_to_PtrOfAlgorithm(cv::Ptr<cv::img_hash::BlockMeanHash>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::img_hash::BlockMeanHash>::to_PtrOfImgHashBase() generated
	// ("cv::Ptr<cv::img_hash::BlockMeanHash>::to_PtrOfImgHashBase", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::img_hash::ImgHashBase>* cv_PtrLcv_img_hash_BlockMeanHashG_to_PtrOfImgHashBase(cv::Ptr<cv::img_hash::BlockMeanHash>* instance) {
			return new cv::Ptr<cv::img_hash::ImgHashBase>(instance->dynamicCast<cv::img_hash::ImgHashBase>());
	}

	// cv::Ptr<cv::img_hash::BlockMeanHash>::new(TraitClass) generated
	// ("cv::Ptr<cv::img_hash::BlockMeanHash>::new", vec![(pred!(const, ["val"], ["cv::img_hash::BlockMeanHash"]), _)]),
	cv::Ptr<cv::img_hash::BlockMeanHash>* cv_PtrLcv_img_hash_BlockMeanHashG_new_const_BlockMeanHash(cv::img_hash::BlockMeanHash* val) {
			return new cv::Ptr<cv::img_hash::BlockMeanHash>(val);
	}

}

