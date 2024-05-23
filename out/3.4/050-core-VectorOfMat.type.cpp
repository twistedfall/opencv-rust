extern "C" {
	// std::vector<cv::Mat>::new() generated
	// ("std::vector<cv::Mat>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Mat>* std_vectorLcv_MatG_new_const() {
			std::vector<cv::Mat>* ret = new std::vector<cv::Mat>();
			return ret;
	}

	// std::vector<cv::Mat>::delete() generated
	// ("std::vector<cv::Mat>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_MatG_delete(std::vector<cv::Mat>* instance) {
			delete instance;
	}

	// std::vector<cv::Mat>::len() generated
	// ("std::vector<cv::Mat>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_MatG_len_const(const std::vector<cv::Mat>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::Mat>::isEmpty() generated
	// ("std::vector<cv::Mat>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_MatG_isEmpty_const(const std::vector<cv::Mat>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::Mat>::capacity() generated
	// ("std::vector<cv::Mat>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_MatG_capacity_const(const std::vector<cv::Mat>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::Mat>::shrinkToFit() generated
	// ("std::vector<cv::Mat>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_MatG_shrinkToFit(std::vector<cv::Mat>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::Mat>::reserve(Primitive) generated
	// ("std::vector<cv::Mat>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_MatG_reserve_size_t(std::vector<cv::Mat>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::Mat>::remove(Primitive) generated
	// ("std::vector<cv::Mat>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_MatG_remove_size_t(std::vector<cv::Mat>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::Mat>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::Mat>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_MatG_swap_size_t_size_t(std::vector<cv::Mat>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::Mat>::clear() generated
	// ("std::vector<cv::Mat>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_MatG_clear(std::vector<cv::Mat>* instance) {
			instance->clear();
	}

	// std::vector<cv::Mat>::push(TraitClass) generated
	// ("std::vector<cv::Mat>::push", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void std_vectorLcv_MatG_push_const_Mat(std::vector<cv::Mat>* instance, const cv::Mat* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::Mat>::insert(Primitive, TraitClass) generated
	// ("std::vector<cv::Mat>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Mat"]), _)]),
	void std_vectorLcv_MatG_insert_size_t_const_Mat(std::vector<cv::Mat>* instance, size_t index, const cv::Mat* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::Mat>::get(Primitive) generated
	// ("std::vector<cv::Mat>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_MatG_get_const_size_t(const std::vector<cv::Mat>* instance, size_t index, cv::Mat** ocvrs_return) {
			cv::Mat ret = (*instance)[index];
			*ocvrs_return = new cv::Mat(ret);
	}

	// std::vector<cv::Mat>::set(Primitive, TraitClass) generated
	// ("std::vector<cv::Mat>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Mat"]), _)]),
	void std_vectorLcv_MatG_set_size_t_const_Mat(std::vector<cv::Mat>* instance, size_t index, const cv::Mat* val) {
			(*instance)[index] = *val;
	}

}


