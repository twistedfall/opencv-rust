extern "C" {
	// std::vector<cv::Vec2i>::new() generated
	// ("std::vector<cv::Vec2i>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Vec2i>* std_vectorLcv_Vec2iG_new_const() {
			std::vector<cv::Vec2i>* ret = new std::vector<cv::Vec2i>();
			return ret;
	}

	// std::vector<cv::Vec2i>::delete() generated
	// ("std::vector<cv::Vec2i>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_Vec2iG_delete(std::vector<cv::Vec2i>* instance) {
			delete instance;
	}

	// std::vector<cv::Vec2i>::len() generated
	// ("std::vector<cv::Vec2i>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_Vec2iG_len_const(const std::vector<cv::Vec2i>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::Vec2i>::isEmpty() generated
	// ("std::vector<cv::Vec2i>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_Vec2iG_isEmpty_const(const std::vector<cv::Vec2i>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::Vec2i>::capacity() generated
	// ("std::vector<cv::Vec2i>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_Vec2iG_capacity_const(const std::vector<cv::Vec2i>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::Vec2i>::shrinkToFit() generated
	// ("std::vector<cv::Vec2i>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_Vec2iG_shrinkToFit(std::vector<cv::Vec2i>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::Vec2i>::reserve(Primitive) generated
	// ("std::vector<cv::Vec2i>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_Vec2iG_reserve_size_t(std::vector<cv::Vec2i>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::Vec2i>::remove(Primitive) generated
	// ("std::vector<cv::Vec2i>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_Vec2iG_remove_size_t(std::vector<cv::Vec2i>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::Vec2i>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::Vec2i>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_Vec2iG_swap_size_t_size_t(std::vector<cv::Vec2i>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::Vec2i>::clear() generated
	// ("std::vector<cv::Vec2i>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_Vec2iG_clear(std::vector<cv::Vec2i>* instance) {
			instance->clear();
	}

	// std::vector<cv::Vec2i>::push(SimpleClass) generated
	// ("std::vector<cv::Vec2i>::push", vec![(pred!(mut, ["val"], ["const cv::Vec2i"]), _)]),
	void std_vectorLcv_Vec2iG_push_const_Vec2i(std::vector<cv::Vec2i>* instance, const cv::Vec2i* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::Vec2i>::insert(Primitive, SimpleClass) generated
	// ("std::vector<cv::Vec2i>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Vec2i"]), _)]),
	void std_vectorLcv_Vec2iG_insert_size_t_const_Vec2i(std::vector<cv::Vec2i>* instance, size_t index, const cv::Vec2i* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::Vec2i>::get(Primitive) generated
	// ("std::vector<cv::Vec2i>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_Vec2iG_get_const_size_t(const std::vector<cv::Vec2i>* instance, size_t index, cv::Vec2i* ocvrs_return) {
			cv::Vec2i ret = (*instance)[index];
			*ocvrs_return = ret;
	}

	// std::vector<cv::Vec2i>::set(Primitive, SimpleClass) generated
	// ("std::vector<cv::Vec2i>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Vec2i"]), _)]),
	void std_vectorLcv_Vec2iG_set_size_t_const_Vec2i(std::vector<cv::Vec2i>* instance, size_t index, const cv::Vec2i* val) {
			(*instance)[index] = *val;
	}

	// std::vector<cv::Vec2i>::clone() generated
	// ("std::vector<cv::Vec2i>::clone", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Vec2i>* std_vectorLcv_Vec2iG_clone_const(const std::vector<cv::Vec2i>* instance) {
			std::vector<cv::Vec2i> ret = std::vector<cv::Vec2i>(*instance);
			return new std::vector<cv::Vec2i>(ret);
	}

	// std::vector<cv::Vec2i>::data() generated
	// ("std::vector<cv::Vec2i>::data", vec![(pred!(const, [], []), _)]),
	const cv::Vec2i* std_vectorLcv_Vec2iG_data_const(const std::vector<cv::Vec2i>* instance) {
			const cv::Vec2i* ret = instance->data();
			return ret;
	}

	// std::vector<cv::Vec2i>::dataMut() generated
	// ("std::vector<cv::Vec2i>::dataMut", vec![(pred!(mut, [], []), _)]),
	cv::Vec2i* std_vectorLcv_Vec2iG_dataMut(std::vector<cv::Vec2i>* instance) {
			cv::Vec2i* ret = instance->data();
			return ret;
	}

	// cv::fromSlice(SimpleClass, Primitive) generated
	// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const cv::Vec2i*", "size_t"]), _)]),
	std::vector<cv::Vec2i>* cv_fromSlice_const_const_Vec2iX_size_t(const cv::Vec2i* data, size_t len) {
			return new std::vector<cv::Vec2i>(data, data + len);
	}

	// std::vector<cv::Vec2i>::inputArray() generated
	// ("std::vector<cv::Vec2i>::inputArray", vec![(pred!(const, [], []), _)]),
	void std_vectorLcv_Vec2iG_inputArray_const(const std::vector<cv::Vec2i>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			const cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new const cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<cv::Vec2i>::outputArray() generated
	// ("std::vector<cv::Vec2i>::outputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_Vec2iG_outputArray(std::vector<cv::Vec2i>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<cv::Vec2i>::inputOutputArray() generated
	// ("std::vector<cv::Vec2i>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_Vec2iG_inputOutputArray(std::vector<cv::Vec2i>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

}


