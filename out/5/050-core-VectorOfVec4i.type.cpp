extern "C" {
	// std::vector<cv::Vec4i>::new() generated
	// ("std::vector<cv::Vec4i>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Vec4i>* std_vectorLcv_Vec4iG_new_const() {
			std::vector<cv::Vec4i>* ret = new std::vector<cv::Vec4i>();
			return ret;
	}

	// std::vector<cv::Vec4i>::delete() generated
	// ("std::vector<cv::Vec4i>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_Vec4iG_delete(std::vector<cv::Vec4i>* instance) {
			delete instance;
	}

	// std::vector<cv::Vec4i>::len() generated
	// ("std::vector<cv::Vec4i>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_Vec4iG_len_const(const std::vector<cv::Vec4i>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::Vec4i>::isEmpty() generated
	// ("std::vector<cv::Vec4i>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_Vec4iG_isEmpty_const(const std::vector<cv::Vec4i>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::Vec4i>::capacity() generated
	// ("std::vector<cv::Vec4i>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_Vec4iG_capacity_const(const std::vector<cv::Vec4i>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::Vec4i>::shrinkToFit() generated
	// ("std::vector<cv::Vec4i>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_Vec4iG_shrinkToFit(std::vector<cv::Vec4i>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::Vec4i>::reserve(Primitive) generated
	// ("std::vector<cv::Vec4i>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_Vec4iG_reserve_size_t(std::vector<cv::Vec4i>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::Vec4i>::remove(Primitive) generated
	// ("std::vector<cv::Vec4i>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_Vec4iG_remove_size_t(std::vector<cv::Vec4i>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::Vec4i>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::Vec4i>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_Vec4iG_swap_size_t_size_t(std::vector<cv::Vec4i>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::Vec4i>::clear() generated
	// ("std::vector<cv::Vec4i>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_Vec4iG_clear(std::vector<cv::Vec4i>* instance) {
			instance->clear();
	}

	// std::vector<cv::Vec4i>::push(SimpleClass) generated
	// ("std::vector<cv::Vec4i>::push", vec![(pred!(mut, ["val"], ["const cv::Vec4i"]), _)]),
	void std_vectorLcv_Vec4iG_push_const_Vec4i(std::vector<cv::Vec4i>* instance, const cv::Vec4i* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::Vec4i>::insert(Primitive, SimpleClass) generated
	// ("std::vector<cv::Vec4i>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Vec4i"]), _)]),
	void std_vectorLcv_Vec4iG_insert_size_t_const_Vec4i(std::vector<cv::Vec4i>* instance, size_t index, const cv::Vec4i* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::Vec4i>::get(Primitive) generated
	// ("std::vector<cv::Vec4i>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_Vec4iG_get_const_size_t(const std::vector<cv::Vec4i>* instance, size_t index, cv::Vec4i* ocvrs_return) {
			cv::Vec4i ret = (*instance)[index];
			*ocvrs_return = ret;
	}

	// std::vector<cv::Vec4i>::set(Primitive, SimpleClass) generated
	// ("std::vector<cv::Vec4i>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Vec4i"]), _)]),
	void std_vectorLcv_Vec4iG_set_size_t_const_Vec4i(std::vector<cv::Vec4i>* instance, size_t index, const cv::Vec4i* val) {
			(*instance)[index] = *val;
	}

	// std::vector<cv::Vec4i>::clone() generated
	// ("std::vector<cv::Vec4i>::clone", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Vec4i>* std_vectorLcv_Vec4iG_clone_const(const std::vector<cv::Vec4i>* instance) {
			std::vector<cv::Vec4i> ret = std::vector<cv::Vec4i>(*instance);
			return new std::vector<cv::Vec4i>(ret);
	}

	// std::vector<cv::Vec4i>::data() generated
	// ("std::vector<cv::Vec4i>::data", vec![(pred!(const, [], []), _)]),
	const cv::Vec4i* std_vectorLcv_Vec4iG_data_const(const std::vector<cv::Vec4i>* instance) {
			const cv::Vec4i* ret = instance->data();
			return ret;
	}

	// std::vector<cv::Vec4i>::dataMut() generated
	// ("std::vector<cv::Vec4i>::dataMut", vec![(pred!(mut, [], []), _)]),
	cv::Vec4i* std_vectorLcv_Vec4iG_dataMut(std::vector<cv::Vec4i>* instance) {
			cv::Vec4i* ret = instance->data();
			return ret;
	}

	// cv::fromSlice(SimpleClass, Primitive) generated
	// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const cv::Vec4i*", "size_t"]), _)]),
	std::vector<cv::Vec4i>* cv_fromSlice_const_const_Vec4iX_size_t(const cv::Vec4i* data, size_t len) {
			return new std::vector<cv::Vec4i>(data, data + len);
	}

	// std::vector<cv::Vec4i>::inputArray() generated
	// ("std::vector<cv::Vec4i>::inputArray", vec![(pred!(const, [], []), _)]),
	void std_vectorLcv_Vec4iG_inputArray_const(const std::vector<cv::Vec4i>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			const cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new const cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<cv::Vec4i>::outputArray() generated
	// ("std::vector<cv::Vec4i>::outputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_Vec4iG_outputArray(std::vector<cv::Vec4i>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<cv::Vec4i>::inputOutputArray() generated
	// ("std::vector<cv::Vec4i>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_Vec4iG_inputOutputArray(std::vector<cv::Vec4i>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

}


