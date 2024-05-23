extern "C" {
	// std::vector<cv::ppf_match_3d::Pose3DPtr>::new() generated
	// ("std::vector<cv::ppf_match_3d::Pose3DPtr>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::ppf_match_3d::Pose3DPtr>* std_vectorLcv_ppf_match_3d_Pose3DPtrG_new_const() {
			std::vector<cv::ppf_match_3d::Pose3DPtr>* ret = new std::vector<cv::ppf_match_3d::Pose3DPtr>();
			return ret;
	}

	// std::vector<cv::ppf_match_3d::Pose3DPtr>::delete() generated
	// ("std::vector<cv::ppf_match_3d::Pose3DPtr>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_ppf_match_3d_Pose3DPtrG_delete(std::vector<cv::ppf_match_3d::Pose3DPtr>* instance) {
			delete instance;
	}

	// std::vector<cv::ppf_match_3d::Pose3DPtr>::len() generated
	// ("std::vector<cv::ppf_match_3d::Pose3DPtr>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_ppf_match_3d_Pose3DPtrG_len_const(const std::vector<cv::ppf_match_3d::Pose3DPtr>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::ppf_match_3d::Pose3DPtr>::isEmpty() generated
	// ("std::vector<cv::ppf_match_3d::Pose3DPtr>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_ppf_match_3d_Pose3DPtrG_isEmpty_const(const std::vector<cv::ppf_match_3d::Pose3DPtr>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::ppf_match_3d::Pose3DPtr>::capacity() generated
	// ("std::vector<cv::ppf_match_3d::Pose3DPtr>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_ppf_match_3d_Pose3DPtrG_capacity_const(const std::vector<cv::ppf_match_3d::Pose3DPtr>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::ppf_match_3d::Pose3DPtr>::shrinkToFit() generated
	// ("std::vector<cv::ppf_match_3d::Pose3DPtr>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_ppf_match_3d_Pose3DPtrG_shrinkToFit(std::vector<cv::ppf_match_3d::Pose3DPtr>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::ppf_match_3d::Pose3DPtr>::reserve(Primitive) generated
	// ("std::vector<cv::ppf_match_3d::Pose3DPtr>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_ppf_match_3d_Pose3DPtrG_reserve_size_t(std::vector<cv::ppf_match_3d::Pose3DPtr>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::ppf_match_3d::Pose3DPtr>::remove(Primitive) generated
	// ("std::vector<cv::ppf_match_3d::Pose3DPtr>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_ppf_match_3d_Pose3DPtrG_remove_size_t(std::vector<cv::ppf_match_3d::Pose3DPtr>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::ppf_match_3d::Pose3DPtr>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::ppf_match_3d::Pose3DPtr>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_ppf_match_3d_Pose3DPtrG_swap_size_t_size_t(std::vector<cv::ppf_match_3d::Pose3DPtr>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::ppf_match_3d::Pose3DPtr>::clear() generated
	// ("std::vector<cv::ppf_match_3d::Pose3DPtr>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_ppf_match_3d_Pose3DPtrG_clear(std::vector<cv::ppf_match_3d::Pose3DPtr>* instance) {
			instance->clear();
	}

	// std::vector<cv::ppf_match_3d::Pose3DPtr>::push(CppPassByVoidPtr) generated
	// ("std::vector<cv::ppf_match_3d::Pose3DPtr>::push", vec![(pred!(mut, ["val"], ["const cv::ppf_match_3d::Pose3DPtr"]), _)]),
	void std_vectorLcv_ppf_match_3d_Pose3DPtrG_push_const_Pose3DPtr(std::vector<cv::ppf_match_3d::Pose3DPtr>* instance, const cv::ppf_match_3d::Pose3DPtr* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::ppf_match_3d::Pose3DPtr>::insert(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<cv::ppf_match_3d::Pose3DPtr>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::ppf_match_3d::Pose3DPtr"]), _)]),
	void std_vectorLcv_ppf_match_3d_Pose3DPtrG_insert_size_t_const_Pose3DPtr(std::vector<cv::ppf_match_3d::Pose3DPtr>* instance, size_t index, const cv::ppf_match_3d::Pose3DPtr* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::ppf_match_3d::Pose3DPtr>::get(Primitive) generated
	// ("std::vector<cv::ppf_match_3d::Pose3DPtr>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_ppf_match_3d_Pose3DPtrG_get_const_size_t(const std::vector<cv::ppf_match_3d::Pose3DPtr>* instance, size_t index, cv::ppf_match_3d::Pose3DPtr** ocvrs_return) {
			cv::ppf_match_3d::Pose3DPtr ret = (*instance)[index];
			*ocvrs_return = new cv::ppf_match_3d::Pose3DPtr(ret);
	}

	// std::vector<cv::ppf_match_3d::Pose3DPtr>::set(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<cv::ppf_match_3d::Pose3DPtr>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::ppf_match_3d::Pose3DPtr"]), _)]),
	void std_vectorLcv_ppf_match_3d_Pose3DPtrG_set_size_t_const_Pose3DPtr(std::vector<cv::ppf_match_3d::Pose3DPtr>* instance, size_t index, const cv::ppf_match_3d::Pose3DPtr* val) {
			(*instance)[index] = *val;
	}

}


