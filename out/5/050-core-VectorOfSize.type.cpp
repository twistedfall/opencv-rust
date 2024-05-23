extern "C" {
	// std::vector<cv::Size>::new() generated
	// ("std::vector<cv::Size>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Size>* std_vectorLcv_SizeG_new_const() {
			std::vector<cv::Size>* ret = new std::vector<cv::Size>();
			return ret;
	}

	// std::vector<cv::Size>::delete() generated
	// ("std::vector<cv::Size>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_SizeG_delete(std::vector<cv::Size>* instance) {
			delete instance;
	}

	// std::vector<cv::Size>::len() generated
	// ("std::vector<cv::Size>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_SizeG_len_const(const std::vector<cv::Size>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::Size>::isEmpty() generated
	// ("std::vector<cv::Size>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_SizeG_isEmpty_const(const std::vector<cv::Size>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::Size>::capacity() generated
	// ("std::vector<cv::Size>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_SizeG_capacity_const(const std::vector<cv::Size>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::Size>::shrinkToFit() generated
	// ("std::vector<cv::Size>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_SizeG_shrinkToFit(std::vector<cv::Size>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::Size>::reserve(Primitive) generated
	// ("std::vector<cv::Size>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_SizeG_reserve_size_t(std::vector<cv::Size>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::Size>::remove(Primitive) generated
	// ("std::vector<cv::Size>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_SizeG_remove_size_t(std::vector<cv::Size>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::Size>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::Size>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_SizeG_swap_size_t_size_t(std::vector<cv::Size>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::Size>::clear() generated
	// ("std::vector<cv::Size>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_SizeG_clear(std::vector<cv::Size>* instance) {
			instance->clear();
	}

	// std::vector<cv::Size>::push(SimpleClass) generated
	// ("std::vector<cv::Size>::push", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	void std_vectorLcv_SizeG_push_const_Size(std::vector<cv::Size>* instance, const cv::Size* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::Size>::insert(Primitive, SimpleClass) generated
	// ("std::vector<cv::Size>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Size"]), _)]),
	void std_vectorLcv_SizeG_insert_size_t_const_Size(std::vector<cv::Size>* instance, size_t index, const cv::Size* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::Size>::get(Primitive) generated
	// ("std::vector<cv::Size>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_SizeG_get_const_size_t(const std::vector<cv::Size>* instance, size_t index, cv::Size* ocvrs_return) {
			cv::Size ret = (*instance)[index];
			*ocvrs_return = ret;
	}

	// std::vector<cv::Size>::set(Primitive, SimpleClass) generated
	// ("std::vector<cv::Size>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Size"]), _)]),
	void std_vectorLcv_SizeG_set_size_t_const_Size(std::vector<cv::Size>* instance, size_t index, const cv::Size* val) {
			(*instance)[index] = *val;
	}

	// std::vector<cv::Size>::clone() generated
	// ("std::vector<cv::Size>::clone", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Size>* std_vectorLcv_SizeG_clone_const(const std::vector<cv::Size>* instance) {
			std::vector<cv::Size> ret = std::vector<cv::Size>(*instance);
			return new std::vector<cv::Size>(ret);
	}

	// std::vector<cv::Size>::data() generated
	// ("std::vector<cv::Size>::data", vec![(pred!(const, [], []), _)]),
	const cv::Size* std_vectorLcv_SizeG_data_const(const std::vector<cv::Size>* instance) {
			const cv::Size* ret = instance->data();
			return ret;
	}

	// std::vector<cv::Size>::dataMut() generated
	// ("std::vector<cv::Size>::dataMut", vec![(pred!(mut, [], []), _)]),
	cv::Size* std_vectorLcv_SizeG_dataMut(std::vector<cv::Size>* instance) {
			cv::Size* ret = instance->data();
			return ret;
	}

	// cv::fromSlice(SimpleClass, Primitive) generated
	// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const cv::Size*", "size_t"]), _)]),
	std::vector<cv::Size>* cv_fromSlice_const_const_SizeX_size_t(const cv::Size* data, size_t len) {
			return new std::vector<cv::Size>(data, data + len);
	}

	// std::vector<cv::Size>::inputArray() generated
	// ("std::vector<cv::Size>::inputArray", vec![(pred!(const, [], []), _)]),
	void std_vectorLcv_SizeG_inputArray_const(const std::vector<cv::Size>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			const cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new const cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<cv::Size>::outputArray() generated
	// ("std::vector<cv::Size>::outputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_SizeG_outputArray(std::vector<cv::Size>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<cv::Size>::inputOutputArray() generated
	// ("std::vector<cv::Size>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_SizeG_inputOutputArray(std::vector<cv::Size>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

}


