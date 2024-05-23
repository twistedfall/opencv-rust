extern "C" {
	// std::vector<cv::Vec6f>::new() generated
	// ("std::vector<cv::Vec6f>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Vec6f>* std_vectorLcv_Vec6fG_new_const() {
			std::vector<cv::Vec6f>* ret = new std::vector<cv::Vec6f>();
			return ret;
	}

	// std::vector<cv::Vec6f>::delete() generated
	// ("std::vector<cv::Vec6f>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_Vec6fG_delete(std::vector<cv::Vec6f>* instance) {
			delete instance;
	}

	// std::vector<cv::Vec6f>::len() generated
	// ("std::vector<cv::Vec6f>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_Vec6fG_len_const(const std::vector<cv::Vec6f>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::Vec6f>::isEmpty() generated
	// ("std::vector<cv::Vec6f>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_Vec6fG_isEmpty_const(const std::vector<cv::Vec6f>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::Vec6f>::capacity() generated
	// ("std::vector<cv::Vec6f>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_Vec6fG_capacity_const(const std::vector<cv::Vec6f>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::Vec6f>::shrinkToFit() generated
	// ("std::vector<cv::Vec6f>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_Vec6fG_shrinkToFit(std::vector<cv::Vec6f>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::Vec6f>::reserve(Primitive) generated
	// ("std::vector<cv::Vec6f>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_Vec6fG_reserve_size_t(std::vector<cv::Vec6f>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::Vec6f>::remove(Primitive) generated
	// ("std::vector<cv::Vec6f>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_Vec6fG_remove_size_t(std::vector<cv::Vec6f>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::Vec6f>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::Vec6f>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_Vec6fG_swap_size_t_size_t(std::vector<cv::Vec6f>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::Vec6f>::clear() generated
	// ("std::vector<cv::Vec6f>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_Vec6fG_clear(std::vector<cv::Vec6f>* instance) {
			instance->clear();
	}

	// std::vector<cv::Vec6f>::push(SimpleClass) generated
	// ("std::vector<cv::Vec6f>::push", vec![(pred!(mut, ["val"], ["const cv::Vec6f"]), _)]),
	void std_vectorLcv_Vec6fG_push_const_Vec6f(std::vector<cv::Vec6f>* instance, const cv::Vec6f* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::Vec6f>::insert(Primitive, SimpleClass) generated
	// ("std::vector<cv::Vec6f>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Vec6f"]), _)]),
	void std_vectorLcv_Vec6fG_insert_size_t_const_Vec6f(std::vector<cv::Vec6f>* instance, size_t index, const cv::Vec6f* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::Vec6f>::get(Primitive) generated
	// ("std::vector<cv::Vec6f>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_Vec6fG_get_const_size_t(const std::vector<cv::Vec6f>* instance, size_t index, cv::Vec6f* ocvrs_return) {
			cv::Vec6f ret = (*instance)[index];
			*ocvrs_return = ret;
	}

	// std::vector<cv::Vec6f>::set(Primitive, SimpleClass) generated
	// ("std::vector<cv::Vec6f>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Vec6f"]), _)]),
	void std_vectorLcv_Vec6fG_set_size_t_const_Vec6f(std::vector<cv::Vec6f>* instance, size_t index, const cv::Vec6f* val) {
			(*instance)[index] = *val;
	}

	// std::vector<cv::Vec6f>::clone() generated
	// ("std::vector<cv::Vec6f>::clone", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Vec6f>* std_vectorLcv_Vec6fG_clone_const(const std::vector<cv::Vec6f>* instance) {
			std::vector<cv::Vec6f> ret = std::vector<cv::Vec6f>(*instance);
			return new std::vector<cv::Vec6f>(ret);
	}

	// std::vector<cv::Vec6f>::data() generated
	// ("std::vector<cv::Vec6f>::data", vec![(pred!(const, [], []), _)]),
	const cv::Vec6f* std_vectorLcv_Vec6fG_data_const(const std::vector<cv::Vec6f>* instance) {
			const cv::Vec6f* ret = instance->data();
			return ret;
	}

	// std::vector<cv::Vec6f>::dataMut() generated
	// ("std::vector<cv::Vec6f>::dataMut", vec![(pred!(mut, [], []), _)]),
	cv::Vec6f* std_vectorLcv_Vec6fG_dataMut(std::vector<cv::Vec6f>* instance) {
			cv::Vec6f* ret = instance->data();
			return ret;
	}

	// cv::fromSlice(SimpleClass, Primitive) generated
	// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const cv::Vec6f*", "size_t"]), _)]),
	std::vector<cv::Vec6f>* cv_fromSlice_const_const_Vec6fX_size_t(const cv::Vec6f* data, size_t len) {
			return new std::vector<cv::Vec6f>(data, data + len);
	}

	// std::vector<cv::Vec6f>::inputArray() generated
	// ("std::vector<cv::Vec6f>::inputArray", vec![(pred!(const, [], []), _)]),
	void std_vectorLcv_Vec6fG_inputArray_const(const std::vector<cv::Vec6f>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			const cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new const cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<cv::Vec6f>::outputArray() generated
	// ("std::vector<cv::Vec6f>::outputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_Vec6fG_outputArray(std::vector<cv::Vec6f>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<cv::Vec6f>::inputOutputArray() generated
	// ("std::vector<cv::Vec6f>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_Vec6fG_inputOutputArray(std::vector<cv::Vec6f>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

}


