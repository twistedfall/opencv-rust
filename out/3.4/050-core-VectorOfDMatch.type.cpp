extern "C" {
	// std::vector<cv::DMatch>::new() generated
	// ("std::vector<cv::DMatch>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::DMatch>* std_vectorLcv_DMatchG_new_const() {
			std::vector<cv::DMatch>* ret = new std::vector<cv::DMatch>();
			return ret;
	}

	// std::vector<cv::DMatch>::delete() generated
	// ("std::vector<cv::DMatch>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_DMatchG_delete(std::vector<cv::DMatch>* instance) {
			delete instance;
	}

	// std::vector<cv::DMatch>::len() generated
	// ("std::vector<cv::DMatch>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_DMatchG_len_const(const std::vector<cv::DMatch>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::DMatch>::isEmpty() generated
	// ("std::vector<cv::DMatch>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_DMatchG_isEmpty_const(const std::vector<cv::DMatch>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::DMatch>::capacity() generated
	// ("std::vector<cv::DMatch>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_DMatchG_capacity_const(const std::vector<cv::DMatch>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::DMatch>::shrinkToFit() generated
	// ("std::vector<cv::DMatch>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_DMatchG_shrinkToFit(std::vector<cv::DMatch>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::DMatch>::reserve(Primitive) generated
	// ("std::vector<cv::DMatch>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_DMatchG_reserve_size_t(std::vector<cv::DMatch>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::DMatch>::remove(Primitive) generated
	// ("std::vector<cv::DMatch>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_DMatchG_remove_size_t(std::vector<cv::DMatch>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::DMatch>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::DMatch>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_DMatchG_swap_size_t_size_t(std::vector<cv::DMatch>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::DMatch>::clear() generated
	// ("std::vector<cv::DMatch>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_DMatchG_clear(std::vector<cv::DMatch>* instance) {
			instance->clear();
	}

	// std::vector<cv::DMatch>::push(SimpleClass) generated
	// ("std::vector<cv::DMatch>::push", vec![(pred!(mut, ["val"], ["const cv::DMatch"]), _)]),
	void std_vectorLcv_DMatchG_push_const_DMatch(std::vector<cv::DMatch>* instance, const cv::DMatch* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::DMatch>::insert(Primitive, SimpleClass) generated
	// ("std::vector<cv::DMatch>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::DMatch"]), _)]),
	void std_vectorLcv_DMatchG_insert_size_t_const_DMatch(std::vector<cv::DMatch>* instance, size_t index, const cv::DMatch* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::DMatch>::get(Primitive) generated
	// ("std::vector<cv::DMatch>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_DMatchG_get_const_size_t(const std::vector<cv::DMatch>* instance, size_t index, cv::DMatch* ocvrs_return) {
			cv::DMatch ret = (*instance)[index];
			*ocvrs_return = ret;
	}

	// std::vector<cv::DMatch>::set(Primitive, SimpleClass) generated
	// ("std::vector<cv::DMatch>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::DMatch"]), _)]),
	void std_vectorLcv_DMatchG_set_size_t_const_DMatch(std::vector<cv::DMatch>* instance, size_t index, const cv::DMatch* val) {
			(*instance)[index] = *val;
	}

	// std::vector<cv::DMatch>::clone() generated
	// ("std::vector<cv::DMatch>::clone", vec![(pred!(const, [], []), _)]),
	std::vector<cv::DMatch>* std_vectorLcv_DMatchG_clone_const(const std::vector<cv::DMatch>* instance) {
			std::vector<cv::DMatch> ret = std::vector<cv::DMatch>(*instance);
			return new std::vector<cv::DMatch>(ret);
	}

	// std::vector<cv::DMatch>::data() generated
	// ("std::vector<cv::DMatch>::data", vec![(pred!(const, [], []), _)]),
	const cv::DMatch* std_vectorLcv_DMatchG_data_const(const std::vector<cv::DMatch>* instance) {
			const cv::DMatch* ret = instance->data();
			return ret;
	}

	// std::vector<cv::DMatch>::dataMut() generated
	// ("std::vector<cv::DMatch>::dataMut", vec![(pred!(mut, [], []), _)]),
	cv::DMatch* std_vectorLcv_DMatchG_dataMut(std::vector<cv::DMatch>* instance) {
			cv::DMatch* ret = instance->data();
			return ret;
	}

	// cv::fromSlice(SimpleClass, Primitive) generated
	// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const cv::DMatch*", "size_t"]), _)]),
	std::vector<cv::DMatch>* cv_fromSlice_const_const_DMatchX_size_t(const cv::DMatch* data, size_t len) {
			return new std::vector<cv::DMatch>(data, data + len);
	}

}


