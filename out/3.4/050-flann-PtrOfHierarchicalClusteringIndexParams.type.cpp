extern "C" {
	// cv::Ptr<cv::flann::HierarchicalClusteringIndexParams>::getInnerPtr() generated
	// ("cv::Ptr<cv::flann::HierarchicalClusteringIndexParams>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::flann::HierarchicalClusteringIndexParams* cv_PtrLcv_flann_HierarchicalClusteringIndexParamsG_getInnerPtr_const(const cv::Ptr<cv::flann::HierarchicalClusteringIndexParams>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::flann::HierarchicalClusteringIndexParams>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::flann::HierarchicalClusteringIndexParams>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::flann::HierarchicalClusteringIndexParams* cv_PtrLcv_flann_HierarchicalClusteringIndexParamsG_getInnerPtrMut(cv::Ptr<cv::flann::HierarchicalClusteringIndexParams>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::flann::HierarchicalClusteringIndexParams>::new_null() generated
	// ("cv::Ptr<cv::flann::HierarchicalClusteringIndexParams>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::flann::HierarchicalClusteringIndexParams>* cv_PtrLcv_flann_HierarchicalClusteringIndexParamsG_new_null_const() {
			return new cv::Ptr<cv::flann::HierarchicalClusteringIndexParams>();
	}

	// cv::Ptr<cv::flann::HierarchicalClusteringIndexParams>::delete() generated
	// ("cv::Ptr<cv::flann::HierarchicalClusteringIndexParams>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_flann_HierarchicalClusteringIndexParamsG_delete(cv::Ptr<cv::flann::HierarchicalClusteringIndexParams>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::flann::HierarchicalClusteringIndexParams>::to_PtrOfIndexParams() generated
	// ("cv::Ptr<cv::flann::HierarchicalClusteringIndexParams>::to_PtrOfIndexParams", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::flann::IndexParams>* cv_PtrLcv_flann_HierarchicalClusteringIndexParamsG_to_PtrOfIndexParams(cv::Ptr<cv::flann::HierarchicalClusteringIndexParams>* instance) {
			return new cv::Ptr<cv::flann::IndexParams>(instance->dynamicCast<cv::flann::IndexParams>());
	}

	// cv::Ptr<cv::flann::HierarchicalClusteringIndexParams>::new(TraitClass) generated
	// ("cv::Ptr<cv::flann::HierarchicalClusteringIndexParams>::new", vec![(pred!(const, ["val"], ["cv::flann::HierarchicalClusteringIndexParams"]), _)]),
	cv::Ptr<cv::flann::HierarchicalClusteringIndexParams>* cv_PtrLcv_flann_HierarchicalClusteringIndexParamsG_new_const_HierarchicalClusteringIndexParams(cv::flann::HierarchicalClusteringIndexParams* val) {
			return new cv::Ptr<cv::flann::HierarchicalClusteringIndexParams>(val);
	}

}

