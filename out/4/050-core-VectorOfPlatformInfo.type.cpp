extern "C" {
	// std::vector<cv::ocl::PlatformInfo>::new() generated
	// ("std::vector<cv::ocl::PlatformInfo>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::ocl::PlatformInfo>* std_vectorLcv_ocl_PlatformInfoG_new_const() {
			std::vector<cv::ocl::PlatformInfo>* ret = new std::vector<cv::ocl::PlatformInfo>();
			return ret;
	}

	// std::vector<cv::ocl::PlatformInfo>::delete() generated
	// ("std::vector<cv::ocl::PlatformInfo>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_ocl_PlatformInfoG_delete(std::vector<cv::ocl::PlatformInfo>* instance) {
			delete instance;
	}

	// std::vector<cv::ocl::PlatformInfo>::len() generated
	// ("std::vector<cv::ocl::PlatformInfo>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_ocl_PlatformInfoG_len_const(const std::vector<cv::ocl::PlatformInfo>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::ocl::PlatformInfo>::isEmpty() generated
	// ("std::vector<cv::ocl::PlatformInfo>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_ocl_PlatformInfoG_isEmpty_const(const std::vector<cv::ocl::PlatformInfo>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::ocl::PlatformInfo>::capacity() generated
	// ("std::vector<cv::ocl::PlatformInfo>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_ocl_PlatformInfoG_capacity_const(const std::vector<cv::ocl::PlatformInfo>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::ocl::PlatformInfo>::shrinkToFit() generated
	// ("std::vector<cv::ocl::PlatformInfo>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_ocl_PlatformInfoG_shrinkToFit(std::vector<cv::ocl::PlatformInfo>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::ocl::PlatformInfo>::reserve(Primitive) generated
	// ("std::vector<cv::ocl::PlatformInfo>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_ocl_PlatformInfoG_reserve_size_t(std::vector<cv::ocl::PlatformInfo>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::ocl::PlatformInfo>::remove(Primitive) generated
	// ("std::vector<cv::ocl::PlatformInfo>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_ocl_PlatformInfoG_remove_size_t(std::vector<cv::ocl::PlatformInfo>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::ocl::PlatformInfo>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::ocl::PlatformInfo>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_ocl_PlatformInfoG_swap_size_t_size_t(std::vector<cv::ocl::PlatformInfo>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::ocl::PlatformInfo>::clear() generated
	// ("std::vector<cv::ocl::PlatformInfo>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_ocl_PlatformInfoG_clear(std::vector<cv::ocl::PlatformInfo>* instance) {
			instance->clear();
	}

	// std::vector<cv::ocl::PlatformInfo>::push(TraitClass) generated
	// ("std::vector<cv::ocl::PlatformInfo>::push", vec![(pred!(mut, ["val"], ["const cv::ocl::PlatformInfo"]), _)]),
	void std_vectorLcv_ocl_PlatformInfoG_push_const_PlatformInfo(std::vector<cv::ocl::PlatformInfo>* instance, const cv::ocl::PlatformInfo* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::ocl::PlatformInfo>::insert(Primitive, TraitClass) generated
	// ("std::vector<cv::ocl::PlatformInfo>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::ocl::PlatformInfo"]), _)]),
	void std_vectorLcv_ocl_PlatformInfoG_insert_size_t_const_PlatformInfo(std::vector<cv::ocl::PlatformInfo>* instance, size_t index, const cv::ocl::PlatformInfo* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::ocl::PlatformInfo>::get(Primitive) generated
	// ("std::vector<cv::ocl::PlatformInfo>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_ocl_PlatformInfoG_get_const_size_t(const std::vector<cv::ocl::PlatformInfo>* instance, size_t index, cv::ocl::PlatformInfo** ocvrs_return) {
			cv::ocl::PlatformInfo ret = (*instance)[index];
			*ocvrs_return = new cv::ocl::PlatformInfo(ret);
	}

	// std::vector<cv::ocl::PlatformInfo>::set(Primitive, TraitClass) generated
	// ("std::vector<cv::ocl::PlatformInfo>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::ocl::PlatformInfo"]), _)]),
	void std_vectorLcv_ocl_PlatformInfoG_set_size_t_const_PlatformInfo(std::vector<cv::ocl::PlatformInfo>* instance, size_t index, const cv::ocl::PlatformInfo* val) {
			(*instance)[index] = *val;
	}

}


