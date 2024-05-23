extern "C" {
	// std::vector<cv::GRunArg>::new() generated
	// ("std::vector<cv::GRunArg>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::GRunArg>* std_vectorLcv_GRunArgG_new_const() {
			std::vector<cv::GRunArg>* ret = new std::vector<cv::GRunArg>();
			return ret;
	}

	// std::vector<cv::GRunArg>::delete() generated
	// ("std::vector<cv::GRunArg>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_GRunArgG_delete(std::vector<cv::GRunArg>* instance) {
			delete instance;
	}

	// std::vector<cv::GRunArg>::len() generated
	// ("std::vector<cv::GRunArg>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_GRunArgG_len_const(const std::vector<cv::GRunArg>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::GRunArg>::isEmpty() generated
	// ("std::vector<cv::GRunArg>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_GRunArgG_isEmpty_const(const std::vector<cv::GRunArg>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::GRunArg>::capacity() generated
	// ("std::vector<cv::GRunArg>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_GRunArgG_capacity_const(const std::vector<cv::GRunArg>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::GRunArg>::shrinkToFit() generated
	// ("std::vector<cv::GRunArg>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_GRunArgG_shrinkToFit(std::vector<cv::GRunArg>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::GRunArg>::reserve(Primitive) generated
	// ("std::vector<cv::GRunArg>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_GRunArgG_reserve_size_t(std::vector<cv::GRunArg>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::GRunArg>::remove(Primitive) generated
	// ("std::vector<cv::GRunArg>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_GRunArgG_remove_size_t(std::vector<cv::GRunArg>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::GRunArg>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::GRunArg>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_GRunArgG_swap_size_t_size_t(std::vector<cv::GRunArg>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::GRunArg>::clear() generated
	// ("std::vector<cv::GRunArg>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_GRunArgG_clear(std::vector<cv::GRunArg>* instance) {
			instance->clear();
	}

	// std::vector<cv::GRunArg>::push(TraitClass) generated
	// ("std::vector<cv::GRunArg>::push", vec![(pred!(mut, ["val"], ["const cv::GRunArg"]), _)]),
	void std_vectorLcv_GRunArgG_push_const_GRunArg(std::vector<cv::GRunArg>* instance, const cv::GRunArg* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::GRunArg>::insert(Primitive, TraitClass) generated
	// ("std::vector<cv::GRunArg>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::GRunArg"]), _)]),
	void std_vectorLcv_GRunArgG_insert_size_t_const_GRunArg(std::vector<cv::GRunArg>* instance, size_t index, const cv::GRunArg* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::GRunArg>::get(Primitive) generated
	// ("std::vector<cv::GRunArg>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_GRunArgG_get_const_size_t(const std::vector<cv::GRunArg>* instance, size_t index, cv::GRunArg** ocvrs_return) {
			cv::GRunArg ret = (*instance)[index];
			*ocvrs_return = new cv::GRunArg(ret);
	}

	// std::vector<cv::GRunArg>::set(Primitive, TraitClass) generated
	// ("std::vector<cv::GRunArg>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::GRunArg"]), _)]),
	void std_vectorLcv_GRunArgG_set_size_t_const_GRunArg(std::vector<cv::GRunArg>* instance, size_t index, const cv::GRunArg* val) {
			(*instance)[index] = *val;
	}

}


