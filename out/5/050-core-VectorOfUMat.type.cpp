extern "C" {
	// std::vector<cv::UMat>::new() generated
	// ("std::vector<cv::UMat>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::UMat>* std_vectorLcv_UMatG_new_const() {
			std::vector<cv::UMat>* ret = new std::vector<cv::UMat>();
			return ret;
	}

	// std::vector<cv::UMat>::delete() generated
	// ("std::vector<cv::UMat>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_UMatG_delete(std::vector<cv::UMat>* instance) {
			delete instance;
	}

	// std::vector<cv::UMat>::len() generated
	// ("std::vector<cv::UMat>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_UMatG_len_const(const std::vector<cv::UMat>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::UMat>::isEmpty() generated
	// ("std::vector<cv::UMat>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_UMatG_isEmpty_const(const std::vector<cv::UMat>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::UMat>::capacity() generated
	// ("std::vector<cv::UMat>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_UMatG_capacity_const(const std::vector<cv::UMat>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::UMat>::shrinkToFit() generated
	// ("std::vector<cv::UMat>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_UMatG_shrinkToFit(std::vector<cv::UMat>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::UMat>::reserve(Primitive) generated
	// ("std::vector<cv::UMat>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_UMatG_reserve_size_t(std::vector<cv::UMat>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::UMat>::remove(Primitive) generated
	// ("std::vector<cv::UMat>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_UMatG_remove_size_t(std::vector<cv::UMat>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::UMat>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::UMat>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_UMatG_swap_size_t_size_t(std::vector<cv::UMat>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::UMat>::clear() generated
	// ("std::vector<cv::UMat>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_UMatG_clear(std::vector<cv::UMat>* instance) {
			instance->clear();
	}

	// std::vector<cv::UMat>::push(TraitClass) generated
	// ("std::vector<cv::UMat>::push", vec![(pred!(mut, ["val"], ["const cv::UMat"]), _)]),
	void std_vectorLcv_UMatG_push_const_UMat(std::vector<cv::UMat>* instance, const cv::UMat* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::UMat>::insert(Primitive, TraitClass) generated
	// ("std::vector<cv::UMat>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::UMat"]), _)]),
	void std_vectorLcv_UMatG_insert_size_t_const_UMat(std::vector<cv::UMat>* instance, size_t index, const cv::UMat* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::UMat>::get(Primitive) generated
	// ("std::vector<cv::UMat>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_UMatG_get_const_size_t(const std::vector<cv::UMat>* instance, size_t index, cv::UMat** ocvrs_return) {
			cv::UMat ret = (*instance)[index];
			*ocvrs_return = new cv::UMat(ret);
	}

	// std::vector<cv::UMat>::set(Primitive, TraitClass) generated
	// ("std::vector<cv::UMat>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::UMat"]), _)]),
	void std_vectorLcv_UMatG_set_size_t_const_UMat(std::vector<cv::UMat>* instance, size_t index, const cv::UMat* val) {
			(*instance)[index] = *val;
	}

}


