extern "C" {
	// cv::Ptr<cv::Octree>::getInnerPtr() generated
	// ("cv::Ptr<cv::Octree>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::Octree* cv_PtrLcv_OctreeG_getInnerPtr_const(const cv::Ptr<cv::Octree>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::Octree>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::Octree>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::Octree* cv_PtrLcv_OctreeG_getInnerPtrMut(cv::Ptr<cv::Octree>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::Octree>::new_null() generated
	// ("cv::Ptr<cv::Octree>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::Octree>* cv_PtrLcv_OctreeG_new_null_const() {
			return new cv::Ptr<cv::Octree>();
	}

	// cv::Ptr<cv::Octree>::delete() generated
	// ("cv::Ptr<cv::Octree>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_OctreeG_delete(cv::Ptr<cv::Octree>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::Octree>::new(TraitClass) generated
	// ("cv::Ptr<cv::Octree>::new", vec![(pred!(const, ["val"], ["cv::Octree"]), _)]),
	cv::Ptr<cv::Octree>* cv_PtrLcv_OctreeG_new_const_Octree(cv::Octree* val) {
			return new cv::Ptr<cv::Octree>(val);
	}

}

