extern "C" {
	// std::vector<cv::GTransform>::new() generated
	// ("std::vector<cv::GTransform>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::GTransform>* std_vectorLcv_GTransformG_new_const() {
			std::vector<cv::GTransform>* ret = new std::vector<cv::GTransform>();
			return ret;
	}

	// std::vector<cv::GTransform>::delete() generated
	// ("std::vector<cv::GTransform>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_GTransformG_delete(std::vector<cv::GTransform>* instance) {
			delete instance;
	}

	// std::vector<cv::GTransform>::len() generated
	// ("std::vector<cv::GTransform>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_GTransformG_len_const(const std::vector<cv::GTransform>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::GTransform>::isEmpty() generated
	// ("std::vector<cv::GTransform>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_GTransformG_isEmpty_const(const std::vector<cv::GTransform>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::GTransform>::capacity() generated
	// ("std::vector<cv::GTransform>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_GTransformG_capacity_const(const std::vector<cv::GTransform>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::GTransform>::shrinkToFit() generated
	// ("std::vector<cv::GTransform>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_GTransformG_shrinkToFit(std::vector<cv::GTransform>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::GTransform>::reserve(Primitive) generated
	// ("std::vector<cv::GTransform>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_GTransformG_reserve_size_t(std::vector<cv::GTransform>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::GTransform>::remove(Primitive) generated
	// ("std::vector<cv::GTransform>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_GTransformG_remove_size_t(std::vector<cv::GTransform>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::GTransform>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::GTransform>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_GTransformG_swap_size_t_size_t(std::vector<cv::GTransform>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::GTransform>::clear() generated
	// ("std::vector<cv::GTransform>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_GTransformG_clear(std::vector<cv::GTransform>* instance) {
			instance->clear();
	}

	// std::vector<cv::GTransform>::push(TraitClass) generated
	// ("std::vector<cv::GTransform>::push", vec![(pred!(mut, ["val"], ["const cv::GTransform"]), _)]),
	void std_vectorLcv_GTransformG_push_const_GTransform(std::vector<cv::GTransform>* instance, const cv::GTransform* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::GTransform>::insert(Primitive, TraitClass) generated
	// ("std::vector<cv::GTransform>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::GTransform"]), _)]),
	void std_vectorLcv_GTransformG_insert_size_t_const_GTransform(std::vector<cv::GTransform>* instance, size_t index, const cv::GTransform* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::GTransform>::get(Primitive) generated
	// ("std::vector<cv::GTransform>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_GTransformG_get_const_size_t(const std::vector<cv::GTransform>* instance, size_t index, cv::GTransform** ocvrs_return) {
			cv::GTransform ret = (*instance)[index];
			*ocvrs_return = new cv::GTransform(ret);
	}

	// std::vector<cv::GTransform>::set(Primitive, TraitClass) generated
	// ("std::vector<cv::GTransform>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::GTransform"]), _)]),
	void std_vectorLcv_GTransformG_set_size_t_const_GTransform(std::vector<cv::GTransform>* instance, size_t index, const cv::GTransform* val) {
			(*instance)[index] = *val;
	}

}


