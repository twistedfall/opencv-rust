extern "C" {
	// cv::Ptr<float>::getInnerPtr() generated
	// ("cv::Ptr<float>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const float* cv_PtrLfloatG_getInnerPtr_const(const cv::Ptr<float>* instance) {
			return instance->get();
	}

	// cv::Ptr<float>::getInnerPtrMut() generated
	// ("cv::Ptr<float>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	float* cv_PtrLfloatG_getInnerPtrMut(cv::Ptr<float>* instance) {
			return instance->get();
	}

	// cv::Ptr<float>::new_null() generated
	// ("cv::Ptr<float>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<float>* cv_PtrLfloatG_new_null_const() {
			return new cv::Ptr<float>();
	}

	// cv::Ptr<float>::delete() generated
	// ("cv::Ptr<float>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLfloatG_delete(cv::Ptr<float>* instance) {
			delete instance;
	}

	// cv::Ptr<float>::new(Primitive) generated
	// ("cv::Ptr<float>::new", vec![(pred!(const, ["val"], ["float"]), _)]),
	cv::Ptr<float>* cv_PtrLfloatG_new_const_float(float val) {
			return new cv::Ptr<float>(new float(val));
	}

}

