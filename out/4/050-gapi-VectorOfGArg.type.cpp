extern "C" {
	// std::vector<cv::GArg>::new() generated
	// ("std::vector<cv::GArg>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::GArg>* std_vectorLcv_GArgG_new_const() {
			std::vector<cv::GArg>* ret = new std::vector<cv::GArg>();
			return ret;
	}

	// std::vector<cv::GArg>::delete() generated
	// ("std::vector<cv::GArg>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_GArgG_delete(std::vector<cv::GArg>* instance) {
			delete instance;
	}

	// std::vector<cv::GArg>::len() generated
	// ("std::vector<cv::GArg>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_GArgG_len_const(const std::vector<cv::GArg>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::GArg>::isEmpty() generated
	// ("std::vector<cv::GArg>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_GArgG_isEmpty_const(const std::vector<cv::GArg>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::GArg>::capacity() generated
	// ("std::vector<cv::GArg>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_GArgG_capacity_const(const std::vector<cv::GArg>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::GArg>::shrinkToFit() generated
	// ("std::vector<cv::GArg>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_GArgG_shrinkToFit(std::vector<cv::GArg>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::GArg>::reserve(Primitive) generated
	// ("std::vector<cv::GArg>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_GArgG_reserve_size_t(std::vector<cv::GArg>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::GArg>::remove(Primitive) generated
	// ("std::vector<cv::GArg>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_GArgG_remove_size_t(std::vector<cv::GArg>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::GArg>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::GArg>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_GArgG_swap_size_t_size_t(std::vector<cv::GArg>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::GArg>::clear() generated
	// ("std::vector<cv::GArg>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_GArgG_clear(std::vector<cv::GArg>* instance) {
			instance->clear();
	}

	// std::vector<cv::GArg>::push(TraitClass) generated
	// ("std::vector<cv::GArg>::push", vec![(pred!(mut, ["val"], ["const cv::GArg"]), _)]),
	void std_vectorLcv_GArgG_push_const_GArg(std::vector<cv::GArg>* instance, const cv::GArg* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::GArg>::insert(Primitive, TraitClass) generated
	// ("std::vector<cv::GArg>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::GArg"]), _)]),
	void std_vectorLcv_GArgG_insert_size_t_const_GArg(std::vector<cv::GArg>* instance, size_t index, const cv::GArg* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::GArg>::get(Primitive) generated
	// ("std::vector<cv::GArg>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_GArgG_get_const_size_t(const std::vector<cv::GArg>* instance, size_t index, cv::GArg** ocvrs_return) {
			cv::GArg ret = (*instance)[index];
			*ocvrs_return = new cv::GArg(ret);
	}

	// std::vector<cv::GArg>::set(Primitive, TraitClass) generated
	// ("std::vector<cv::GArg>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::GArg"]), _)]),
	void std_vectorLcv_GArgG_set_size_t_const_GArg(std::vector<cv::GArg>* instance, size_t index, const cv::GArg* val) {
			(*instance)[index] = *val;
	}

}


