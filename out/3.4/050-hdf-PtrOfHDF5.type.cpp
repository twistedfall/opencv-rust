extern "C" {
	// cv::Ptr<cv::hdf::HDF5>::getInnerPtr() generated
	// ("cv::Ptr<cv::hdf::HDF5>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::hdf::HDF5* cv_PtrLcv_hdf_HDF5G_getInnerPtr_const(const cv::Ptr<cv::hdf::HDF5>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::hdf::HDF5>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::hdf::HDF5>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::hdf::HDF5* cv_PtrLcv_hdf_HDF5G_getInnerPtrMut(cv::Ptr<cv::hdf::HDF5>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::hdf::HDF5>::new_null() generated
	// ("cv::Ptr<cv::hdf::HDF5>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::hdf::HDF5>* cv_PtrLcv_hdf_HDF5G_new_null_const() {
			return new cv::Ptr<cv::hdf::HDF5>();
	}

	// cv::Ptr<cv::hdf::HDF5>::delete() generated
	// ("cv::Ptr<cv::hdf::HDF5>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_hdf_HDF5G_delete(cv::Ptr<cv::hdf::HDF5>* instance) {
			delete instance;
	}

}

