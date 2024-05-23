extern "C" {
	// std::vector<cv::ximgproc::Box>::new() generated
	// ("std::vector<cv::ximgproc::Box>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::ximgproc::Box>* std_vectorLcv_ximgproc_BoxG_new_const() {
			std::vector<cv::ximgproc::Box>* ret = new std::vector<cv::ximgproc::Box>();
			return ret;
	}

	// std::vector<cv::ximgproc::Box>::delete() generated
	// ("std::vector<cv::ximgproc::Box>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_ximgproc_BoxG_delete(std::vector<cv::ximgproc::Box>* instance) {
			delete instance;
	}

	// std::vector<cv::ximgproc::Box>::len() generated
	// ("std::vector<cv::ximgproc::Box>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_ximgproc_BoxG_len_const(const std::vector<cv::ximgproc::Box>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::ximgproc::Box>::isEmpty() generated
	// ("std::vector<cv::ximgproc::Box>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_ximgproc_BoxG_isEmpty_const(const std::vector<cv::ximgproc::Box>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::ximgproc::Box>::capacity() generated
	// ("std::vector<cv::ximgproc::Box>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_ximgproc_BoxG_capacity_const(const std::vector<cv::ximgproc::Box>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::ximgproc::Box>::shrinkToFit() generated
	// ("std::vector<cv::ximgproc::Box>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_ximgproc_BoxG_shrinkToFit(std::vector<cv::ximgproc::Box>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::ximgproc::Box>::reserve(Primitive) generated
	// ("std::vector<cv::ximgproc::Box>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_ximgproc_BoxG_reserve_size_t(std::vector<cv::ximgproc::Box>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::ximgproc::Box>::remove(Primitive) generated
	// ("std::vector<cv::ximgproc::Box>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_ximgproc_BoxG_remove_size_t(std::vector<cv::ximgproc::Box>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::ximgproc::Box>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::ximgproc::Box>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_ximgproc_BoxG_swap_size_t_size_t(std::vector<cv::ximgproc::Box>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::ximgproc::Box>::clear() generated
	// ("std::vector<cv::ximgproc::Box>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_ximgproc_BoxG_clear(std::vector<cv::ximgproc::Box>* instance) {
			instance->clear();
	}

	// std::vector<cv::ximgproc::Box>::push(SimpleClass) generated
	// ("std::vector<cv::ximgproc::Box>::push", vec![(pred!(mut, ["val"], ["const cv::ximgproc::Box"]), _)]),
	void std_vectorLcv_ximgproc_BoxG_push_const_Box(std::vector<cv::ximgproc::Box>* instance, const cv::ximgproc::Box* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::ximgproc::Box>::insert(Primitive, SimpleClass) generated
	// ("std::vector<cv::ximgproc::Box>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::ximgproc::Box"]), _)]),
	void std_vectorLcv_ximgproc_BoxG_insert_size_t_const_Box(std::vector<cv::ximgproc::Box>* instance, size_t index, const cv::ximgproc::Box* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::ximgproc::Box>::get(Primitive) generated
	// ("std::vector<cv::ximgproc::Box>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_ximgproc_BoxG_get_const_size_t(const std::vector<cv::ximgproc::Box>* instance, size_t index, cv::ximgproc::Box* ocvrs_return) {
			cv::ximgproc::Box ret = (*instance)[index];
			*ocvrs_return = ret;
	}

	// std::vector<cv::ximgproc::Box>::set(Primitive, SimpleClass) generated
	// ("std::vector<cv::ximgproc::Box>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::ximgproc::Box"]), _)]),
	void std_vectorLcv_ximgproc_BoxG_set_size_t_const_Box(std::vector<cv::ximgproc::Box>* instance, size_t index, const cv::ximgproc::Box* val) {
			(*instance)[index] = *val;
	}

	// std::vector<cv::ximgproc::Box>::clone() generated
	// ("std::vector<cv::ximgproc::Box>::clone", vec![(pred!(const, [], []), _)]),
	std::vector<cv::ximgproc::Box>* std_vectorLcv_ximgproc_BoxG_clone_const(const std::vector<cv::ximgproc::Box>* instance) {
			std::vector<cv::ximgproc::Box> ret = std::vector<cv::ximgproc::Box>(*instance);
			return new std::vector<cv::ximgproc::Box>(ret);
	}

	// std::vector<cv::ximgproc::Box>::data() generated
	// ("std::vector<cv::ximgproc::Box>::data", vec![(pred!(const, [], []), _)]),
	const cv::ximgproc::Box* std_vectorLcv_ximgproc_BoxG_data_const(const std::vector<cv::ximgproc::Box>* instance) {
			const cv::ximgproc::Box* ret = instance->data();
			return ret;
	}

	// std::vector<cv::ximgproc::Box>::dataMut() generated
	// ("std::vector<cv::ximgproc::Box>::dataMut", vec![(pred!(mut, [], []), _)]),
	cv::ximgproc::Box* std_vectorLcv_ximgproc_BoxG_dataMut(std::vector<cv::ximgproc::Box>* instance) {
			cv::ximgproc::Box* ret = instance->data();
			return ret;
	}

	// cv::fromSlice(SimpleClass, Primitive) generated
	// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const cv::ximgproc::Box*", "size_t"]), _)]),
	std::vector<cv::ximgproc::Box>* cv_fromSlice_const_const_BoxX_size_t(const cv::ximgproc::Box* data, size_t len) {
			return new std::vector<cv::ximgproc::Box>(data, data + len);
	}

}


