extern "C" {
	// std::vector<cv::ConfidenceMap>::new() generated
	// ("std::vector<cv::ConfidenceMap>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::ConfidenceMap>* std_vectorLcv_ConfidenceMapG_new_const() {
			std::vector<cv::ConfidenceMap>* ret = new std::vector<cv::ConfidenceMap>();
			return ret;
	}

	// std::vector<cv::ConfidenceMap>::delete() generated
	// ("std::vector<cv::ConfidenceMap>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_ConfidenceMapG_delete(std::vector<cv::ConfidenceMap>* instance) {
			delete instance;
	}

	// std::vector<cv::ConfidenceMap>::len() generated
	// ("std::vector<cv::ConfidenceMap>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_ConfidenceMapG_len_const(const std::vector<cv::ConfidenceMap>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::ConfidenceMap>::isEmpty() generated
	// ("std::vector<cv::ConfidenceMap>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_ConfidenceMapG_isEmpty_const(const std::vector<cv::ConfidenceMap>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::ConfidenceMap>::capacity() generated
	// ("std::vector<cv::ConfidenceMap>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_ConfidenceMapG_capacity_const(const std::vector<cv::ConfidenceMap>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::ConfidenceMap>::shrinkToFit() generated
	// ("std::vector<cv::ConfidenceMap>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_ConfidenceMapG_shrinkToFit(std::vector<cv::ConfidenceMap>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::ConfidenceMap>::reserve(Primitive) generated
	// ("std::vector<cv::ConfidenceMap>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_ConfidenceMapG_reserve_size_t(std::vector<cv::ConfidenceMap>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::ConfidenceMap>::remove(Primitive) generated
	// ("std::vector<cv::ConfidenceMap>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_ConfidenceMapG_remove_size_t(std::vector<cv::ConfidenceMap>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::ConfidenceMap>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::ConfidenceMap>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_ConfidenceMapG_swap_size_t_size_t(std::vector<cv::ConfidenceMap>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::ConfidenceMap>::clear() generated
	// ("std::vector<cv::ConfidenceMap>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_ConfidenceMapG_clear(std::vector<cv::ConfidenceMap>* instance) {
			instance->clear();
	}

	// std::vector<cv::ConfidenceMap>::push(CppPassByVoidPtr) generated
	// ("std::vector<cv::ConfidenceMap>::push", vec![(pred!(mut, ["val"], ["const cv::ConfidenceMap"]), _)]),
	void std_vectorLcv_ConfidenceMapG_push_const_ConfidenceMap(std::vector<cv::ConfidenceMap>* instance, const cv::ConfidenceMap* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::ConfidenceMap>::insert(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<cv::ConfidenceMap>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::ConfidenceMap"]), _)]),
	void std_vectorLcv_ConfidenceMapG_insert_size_t_const_ConfidenceMap(std::vector<cv::ConfidenceMap>* instance, size_t index, const cv::ConfidenceMap* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::ConfidenceMap>::get(Primitive) generated
	// ("std::vector<cv::ConfidenceMap>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_ConfidenceMapG_get_const_size_t(const std::vector<cv::ConfidenceMap>* instance, size_t index, cv::ConfidenceMap** ocvrs_return) {
			cv::ConfidenceMap ret = (*instance)[index];
			*ocvrs_return = new cv::ConfidenceMap(ret);
	}

	// std::vector<cv::ConfidenceMap>::set(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<cv::ConfidenceMap>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::ConfidenceMap"]), _)]),
	void std_vectorLcv_ConfidenceMapG_set_size_t_const_ConfidenceMap(std::vector<cv::ConfidenceMap>* instance, size_t index, const cv::ConfidenceMap* val) {
			(*instance)[index] = *val;
	}

}


