extern "C" {
	// std::vector<cv::Range>::new() generated
	// ("std::vector<cv::Range>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Range>* std_vectorLcv_RangeG_new_const() {
			std::vector<cv::Range>* ret = new std::vector<cv::Range>();
			return ret;
	}

	// std::vector<cv::Range>::delete() generated
	// ("std::vector<cv::Range>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_RangeG_delete(std::vector<cv::Range>* instance) {
			delete instance;
	}

	// std::vector<cv::Range>::len() generated
	// ("std::vector<cv::Range>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_RangeG_len_const(const std::vector<cv::Range>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::Range>::isEmpty() generated
	// ("std::vector<cv::Range>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_RangeG_isEmpty_const(const std::vector<cv::Range>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::Range>::capacity() generated
	// ("std::vector<cv::Range>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_RangeG_capacity_const(const std::vector<cv::Range>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::Range>::shrinkToFit() generated
	// ("std::vector<cv::Range>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_RangeG_shrinkToFit(std::vector<cv::Range>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::Range>::reserve(Primitive) generated
	// ("std::vector<cv::Range>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_RangeG_reserve_size_t(std::vector<cv::Range>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::Range>::remove(Primitive) generated
	// ("std::vector<cv::Range>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_RangeG_remove_size_t(std::vector<cv::Range>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::Range>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::Range>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_RangeG_swap_size_t_size_t(std::vector<cv::Range>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::Range>::clear() generated
	// ("std::vector<cv::Range>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_RangeG_clear(std::vector<cv::Range>* instance) {
			instance->clear();
	}

	// std::vector<cv::Range>::push(TraitClass) generated
	// ("std::vector<cv::Range>::push", vec![(pred!(mut, ["val"], ["const cv::Range"]), _)]),
	void std_vectorLcv_RangeG_push_const_Range(std::vector<cv::Range>* instance, const cv::Range* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::Range>::insert(Primitive, TraitClass) generated
	// ("std::vector<cv::Range>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Range"]), _)]),
	void std_vectorLcv_RangeG_insert_size_t_const_Range(std::vector<cv::Range>* instance, size_t index, const cv::Range* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::Range>::get(Primitive) generated
	// ("std::vector<cv::Range>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_RangeG_get_const_size_t(const std::vector<cv::Range>* instance, size_t index, cv::Range** ocvrs_return) {
			cv::Range ret = (*instance)[index];
			*ocvrs_return = new cv::Range(ret);
	}

	// std::vector<cv::Range>::set(Primitive, TraitClass) generated
	// ("std::vector<cv::Range>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Range"]), _)]),
	void std_vectorLcv_RangeG_set_size_t_const_Range(std::vector<cv::Range>* instance, size_t index, const cv::Range* val) {
			(*instance)[index] = *val;
	}

}


