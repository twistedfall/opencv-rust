extern "C" {
	// std::vector<cv::GCompileArg>::new() generated
	// ("std::vector<cv::GCompileArg>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::GCompileArg>* std_vectorLcv_GCompileArgG_new_const() {
			std::vector<cv::GCompileArg>* ret = new std::vector<cv::GCompileArg>();
			return ret;
	}

	// std::vector<cv::GCompileArg>::delete() generated
	// ("std::vector<cv::GCompileArg>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_GCompileArgG_delete(std::vector<cv::GCompileArg>* instance) {
			delete instance;
	}

	// std::vector<cv::GCompileArg>::len() generated
	// ("std::vector<cv::GCompileArg>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_GCompileArgG_len_const(const std::vector<cv::GCompileArg>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::GCompileArg>::isEmpty() generated
	// ("std::vector<cv::GCompileArg>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_GCompileArgG_isEmpty_const(const std::vector<cv::GCompileArg>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::GCompileArg>::capacity() generated
	// ("std::vector<cv::GCompileArg>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_GCompileArgG_capacity_const(const std::vector<cv::GCompileArg>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::GCompileArg>::shrinkToFit() generated
	// ("std::vector<cv::GCompileArg>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_GCompileArgG_shrinkToFit(std::vector<cv::GCompileArg>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::GCompileArg>::reserve(Primitive) generated
	// ("std::vector<cv::GCompileArg>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_GCompileArgG_reserve_size_t(std::vector<cv::GCompileArg>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::GCompileArg>::remove(Primitive) generated
	// ("std::vector<cv::GCompileArg>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_GCompileArgG_remove_size_t(std::vector<cv::GCompileArg>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::GCompileArg>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::GCompileArg>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_GCompileArgG_swap_size_t_size_t(std::vector<cv::GCompileArg>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::GCompileArg>::clear() generated
	// ("std::vector<cv::GCompileArg>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_GCompileArgG_clear(std::vector<cv::GCompileArg>* instance) {
			instance->clear();
	}

	// std::vector<cv::GCompileArg>::push(TraitClass) generated
	// ("std::vector<cv::GCompileArg>::push", vec![(pred!(mut, ["val"], ["const cv::GCompileArg"]), _)]),
	void std_vectorLcv_GCompileArgG_push_const_GCompileArg(std::vector<cv::GCompileArg>* instance, const cv::GCompileArg* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::GCompileArg>::insert(Primitive, TraitClass) generated
	// ("std::vector<cv::GCompileArg>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::GCompileArg"]), _)]),
	void std_vectorLcv_GCompileArgG_insert_size_t_const_GCompileArg(std::vector<cv::GCompileArg>* instance, size_t index, const cv::GCompileArg* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::GCompileArg>::get(Primitive) generated
	// ("std::vector<cv::GCompileArg>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_GCompileArgG_get_const_size_t(const std::vector<cv::GCompileArg>* instance, size_t index, cv::GCompileArg** ocvrs_return) {
			cv::GCompileArg ret = (*instance)[index];
			*ocvrs_return = new cv::GCompileArg(ret);
	}

	// std::vector<cv::GCompileArg>::set(Primitive, TraitClass) generated
	// ("std::vector<cv::GCompileArg>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::GCompileArg"]), _)]),
	void std_vectorLcv_GCompileArgG_set_size_t_const_GCompileArg(std::vector<cv::GCompileArg>* instance, size_t index, const cv::GCompileArg* val) {
			(*instance)[index] = *val;
	}

}


