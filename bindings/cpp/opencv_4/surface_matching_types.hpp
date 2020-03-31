template struct Result<cv::Matx<double, 4, 4>>;
template struct Result<cv::Ptr<cv::ppf_match_3d::Pose3D>*>;
template struct Result<cv::Ptr<cv::ppf_match_3d::PoseCluster3D>*>;
template struct Result<cv::Vec<double, 3>>;
template struct Result<cv::Vec<double, 4>>;
template struct Result<cv::ppf_match_3d::ICP*>;
template struct Result<cv::ppf_match_3d::PPF3DDetector*>;
template struct Result<cv::ppf_match_3d::Pose3D*>;
template struct Result<cv::ppf_match_3d::PoseCluster3D*>;
template struct Result<double>;
template struct Result<int>;
template struct Result<std::vector<cv::Ptr<cv::ppf_match_3d::Pose3D>>*>;
template struct Result<unsigned long>;
extern "C" void cv_PtrOfPose3D_delete(cv::Ptr<cv::ppf_match_3d::Pose3D>* instance) {
	delete instance;
}

extern "C" cv::ppf_match_3d::Pose3D* cv_PtrOfPose3D_get_inner_ptr(cv::Ptr<cv::ppf_match_3d::Pose3D>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfPoseCluster3D_delete(cv::Ptr<cv::ppf_match_3d::PoseCluster3D>* instance) {
	delete instance;
}

extern "C" cv::ppf_match_3d::PoseCluster3D* cv_PtrOfPoseCluster3D_get_inner_ptr(cv::Ptr<cv::ppf_match_3d::PoseCluster3D>* instance) {
	return instance->get();
}

extern "C" {
	void cv_VectorOfPose3DPtr_delete(std::vector<cv::ppf_match_3d::Pose3DPtr>* instance) {
		delete instance;
	}

	std::vector<cv::ppf_match_3d::Pose3DPtr>* cv_VectorOfPose3DPtr_new() {
		return new std::vector<cv::ppf_match_3d::Pose3DPtr>();
	}

	size_t cv_VectorOfPose3DPtr_len(const std::vector<cv::ppf_match_3d::Pose3DPtr>* instance) {
		return instance->size();
	}

	bool cv_VectorOfPose3DPtr_is_empty(const std::vector<cv::ppf_match_3d::Pose3DPtr>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfPose3DPtr_capacity(const std::vector<cv::ppf_match_3d::Pose3DPtr>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfPose3DPtr_shrink_to_fit(std::vector<cv::ppf_match_3d::Pose3DPtr>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfPose3DPtr_reserve(std::vector<cv::ppf_match_3d::Pose3DPtr>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfPose3DPtr_remove(std::vector<cv::ppf_match_3d::Pose3DPtr>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfPose3DPtr_swap(std::vector<cv::ppf_match_3d::Pose3DPtr>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfPose3DPtr_clear(std::vector<cv::ppf_match_3d::Pose3DPtr>* instance) {
		instance->clear();
	}

	void cv_VectorOfPose3DPtr_push(std::vector<cv::ppf_match_3d::Pose3DPtr>* instance, cv::ppf_match_3d::Pose3DPtr* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfPose3DPtr_insert(std::vector<cv::ppf_match_3d::Pose3DPtr>* instance, size_t index, cv::ppf_match_3d::Pose3DPtr* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<cv::ppf_match_3d::Pose3DPtr*> cv_VectorOfPose3DPtr_get(const std::vector<cv::ppf_match_3d::Pose3DPtr>* instance, size_t index) {
		return Ok<cv::ppf_match_3d::Pose3DPtr*>(new cv::ppf_match_3d::Pose3DPtr((*instance)[index]));
	}

	void cv_VectorOfPose3DPtr_set(std::vector<cv::ppf_match_3d::Pose3DPtr>* instance, size_t index, cv::ppf_match_3d::Pose3DPtr* val) {
		(*instance)[index] = *val;
	}

}


