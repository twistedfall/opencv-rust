extern "C" {
	// std::vector<cv::ml::DTrees::Split>::new() generated
	// ("std::vector<cv::ml::DTrees::Split>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::ml::DTrees::Split>* std_vectorLcv_ml_DTrees_SplitG_new_const() {
			std::vector<cv::ml::DTrees::Split>* ret = new std::vector<cv::ml::DTrees::Split>();
			return ret;
	}

	// std::vector<cv::ml::DTrees::Split>::delete() generated
	// ("std::vector<cv::ml::DTrees::Split>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_ml_DTrees_SplitG_delete(std::vector<cv::ml::DTrees::Split>* instance) {
			delete instance;
	}

	// std::vector<cv::ml::DTrees::Split>::len() generated
	// ("std::vector<cv::ml::DTrees::Split>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_ml_DTrees_SplitG_len_const(const std::vector<cv::ml::DTrees::Split>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::ml::DTrees::Split>::isEmpty() generated
	// ("std::vector<cv::ml::DTrees::Split>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_ml_DTrees_SplitG_isEmpty_const(const std::vector<cv::ml::DTrees::Split>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::ml::DTrees::Split>::capacity() generated
	// ("std::vector<cv::ml::DTrees::Split>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_ml_DTrees_SplitG_capacity_const(const std::vector<cv::ml::DTrees::Split>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::ml::DTrees::Split>::shrinkToFit() generated
	// ("std::vector<cv::ml::DTrees::Split>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_ml_DTrees_SplitG_shrinkToFit(std::vector<cv::ml::DTrees::Split>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::ml::DTrees::Split>::reserve(Primitive) generated
	// ("std::vector<cv::ml::DTrees::Split>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_ml_DTrees_SplitG_reserve_size_t(std::vector<cv::ml::DTrees::Split>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::ml::DTrees::Split>::remove(Primitive) generated
	// ("std::vector<cv::ml::DTrees::Split>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_ml_DTrees_SplitG_remove_size_t(std::vector<cv::ml::DTrees::Split>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::ml::DTrees::Split>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::ml::DTrees::Split>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_ml_DTrees_SplitG_swap_size_t_size_t(std::vector<cv::ml::DTrees::Split>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::ml::DTrees::Split>::clear() generated
	// ("std::vector<cv::ml::DTrees::Split>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_ml_DTrees_SplitG_clear(std::vector<cv::ml::DTrees::Split>* instance) {
			instance->clear();
	}

	// std::vector<cv::ml::DTrees::Split>::push(TraitClass) generated
	// ("std::vector<cv::ml::DTrees::Split>::push", vec![(pred!(mut, ["val"], ["const cv::ml::DTrees::Split"]), _)]),
	void std_vectorLcv_ml_DTrees_SplitG_push_const_Split(std::vector<cv::ml::DTrees::Split>* instance, const cv::ml::DTrees::Split* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::ml::DTrees::Split>::insert(Primitive, TraitClass) generated
	// ("std::vector<cv::ml::DTrees::Split>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::ml::DTrees::Split"]), _)]),
	void std_vectorLcv_ml_DTrees_SplitG_insert_size_t_const_Split(std::vector<cv::ml::DTrees::Split>* instance, size_t index, const cv::ml::DTrees::Split* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::ml::DTrees::Split>::get(Primitive) generated
	// ("std::vector<cv::ml::DTrees::Split>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_ml_DTrees_SplitG_get_const_size_t(const std::vector<cv::ml::DTrees::Split>* instance, size_t index, cv::ml::DTrees::Split** ocvrs_return) {
			cv::ml::DTrees::Split ret = (*instance)[index];
			*ocvrs_return = new cv::ml::DTrees::Split(ret);
	}

	// std::vector<cv::ml::DTrees::Split>::set(Primitive, TraitClass) generated
	// ("std::vector<cv::ml::DTrees::Split>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::ml::DTrees::Split"]), _)]),
	void std_vectorLcv_ml_DTrees_SplitG_set_size_t_const_Split(std::vector<cv::ml::DTrees::Split>* instance, size_t index, const cv::ml::DTrees::Split* val) {
			(*instance)[index] = *val;
	}

}


