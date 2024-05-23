extern "C" {
	// std::vector<cv::KeyPoint>::new() generated
	// ("std::vector<cv::KeyPoint>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::KeyPoint>* std_vectorLcv_KeyPointG_new_const() {
			std::vector<cv::KeyPoint>* ret = new std::vector<cv::KeyPoint>();
			return ret;
	}

	// std::vector<cv::KeyPoint>::delete() generated
	// ("std::vector<cv::KeyPoint>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_KeyPointG_delete(std::vector<cv::KeyPoint>* instance) {
			delete instance;
	}

	// std::vector<cv::KeyPoint>::len() generated
	// ("std::vector<cv::KeyPoint>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_KeyPointG_len_const(const std::vector<cv::KeyPoint>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::KeyPoint>::isEmpty() generated
	// ("std::vector<cv::KeyPoint>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_KeyPointG_isEmpty_const(const std::vector<cv::KeyPoint>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::KeyPoint>::capacity() generated
	// ("std::vector<cv::KeyPoint>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_KeyPointG_capacity_const(const std::vector<cv::KeyPoint>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::KeyPoint>::shrinkToFit() generated
	// ("std::vector<cv::KeyPoint>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_KeyPointG_shrinkToFit(std::vector<cv::KeyPoint>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::KeyPoint>::reserve(Primitive) generated
	// ("std::vector<cv::KeyPoint>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_KeyPointG_reserve_size_t(std::vector<cv::KeyPoint>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::KeyPoint>::remove(Primitive) generated
	// ("std::vector<cv::KeyPoint>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_KeyPointG_remove_size_t(std::vector<cv::KeyPoint>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::KeyPoint>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::KeyPoint>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_KeyPointG_swap_size_t_size_t(std::vector<cv::KeyPoint>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::KeyPoint>::clear() generated
	// ("std::vector<cv::KeyPoint>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_KeyPointG_clear(std::vector<cv::KeyPoint>* instance) {
			instance->clear();
	}

	// std::vector<cv::KeyPoint>::push(TraitClass) generated
	// ("std::vector<cv::KeyPoint>::push", vec![(pred!(mut, ["val"], ["const cv::KeyPoint"]), _)]),
	void std_vectorLcv_KeyPointG_push_const_KeyPoint(std::vector<cv::KeyPoint>* instance, const cv::KeyPoint* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::KeyPoint>::insert(Primitive, TraitClass) generated
	// ("std::vector<cv::KeyPoint>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::KeyPoint"]), _)]),
	void std_vectorLcv_KeyPointG_insert_size_t_const_KeyPoint(std::vector<cv::KeyPoint>* instance, size_t index, const cv::KeyPoint* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::KeyPoint>::get(Primitive) generated
	// ("std::vector<cv::KeyPoint>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_KeyPointG_get_const_size_t(const std::vector<cv::KeyPoint>* instance, size_t index, cv::KeyPoint** ocvrs_return) {
			cv::KeyPoint ret = (*instance)[index];
			*ocvrs_return = new cv::KeyPoint(ret);
	}

	// std::vector<cv::KeyPoint>::set(Primitive, TraitClass) generated
	// ("std::vector<cv::KeyPoint>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::KeyPoint"]), _)]),
	void std_vectorLcv_KeyPointG_set_size_t_const_KeyPoint(std::vector<cv::KeyPoint>* instance, size_t index, const cv::KeyPoint* val) {
			(*instance)[index] = *val;
	}

}


