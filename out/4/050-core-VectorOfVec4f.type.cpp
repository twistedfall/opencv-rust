extern "C" {
	// std::vector<cv::Vec4f>::new() generated
	// ("std::vector<cv::Vec4f>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Vec4f>* std_vectorLcv_Vec4fG_new_const() {
			std::vector<cv::Vec4f>* ret = new std::vector<cv::Vec4f>();
			return ret;
	}

	// std::vector<cv::Vec4f>::delete() generated
	// ("std::vector<cv::Vec4f>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_Vec4fG_delete(std::vector<cv::Vec4f>* instance) {
			delete instance;
	}

	// std::vector<cv::Vec4f>::len() generated
	// ("std::vector<cv::Vec4f>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_Vec4fG_len_const(const std::vector<cv::Vec4f>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::Vec4f>::isEmpty() generated
	// ("std::vector<cv::Vec4f>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_Vec4fG_isEmpty_const(const std::vector<cv::Vec4f>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::Vec4f>::capacity() generated
	// ("std::vector<cv::Vec4f>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_Vec4fG_capacity_const(const std::vector<cv::Vec4f>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::Vec4f>::shrinkToFit() generated
	// ("std::vector<cv::Vec4f>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_Vec4fG_shrinkToFit(std::vector<cv::Vec4f>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::Vec4f>::reserve(Primitive) generated
	// ("std::vector<cv::Vec4f>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_Vec4fG_reserve_size_t(std::vector<cv::Vec4f>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::Vec4f>::remove(Primitive) generated
	// ("std::vector<cv::Vec4f>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_Vec4fG_remove_size_t(std::vector<cv::Vec4f>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::Vec4f>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::Vec4f>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_Vec4fG_swap_size_t_size_t(std::vector<cv::Vec4f>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::Vec4f>::clear() generated
	// ("std::vector<cv::Vec4f>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_Vec4fG_clear(std::vector<cv::Vec4f>* instance) {
			instance->clear();
	}

	// std::vector<cv::Vec4f>::push(SimpleClass) generated
	// ("std::vector<cv::Vec4f>::push", vec![(pred!(mut, ["val"], ["const cv::Vec4f"]), _)]),
	void std_vectorLcv_Vec4fG_push_const_Vec4f(std::vector<cv::Vec4f>* instance, const cv::Vec4f* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::Vec4f>::insert(Primitive, SimpleClass) generated
	// ("std::vector<cv::Vec4f>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Vec4f"]), _)]),
	void std_vectorLcv_Vec4fG_insert_size_t_const_Vec4f(std::vector<cv::Vec4f>* instance, size_t index, const cv::Vec4f* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::Vec4f>::get(Primitive) generated
	// ("std::vector<cv::Vec4f>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_Vec4fG_get_const_size_t(const std::vector<cv::Vec4f>* instance, size_t index, cv::Vec4f* ocvrs_return) {
			cv::Vec4f ret = (*instance)[index];
			*ocvrs_return = ret;
	}

	// std::vector<cv::Vec4f>::set(Primitive, SimpleClass) generated
	// ("std::vector<cv::Vec4f>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Vec4f"]), _)]),
	void std_vectorLcv_Vec4fG_set_size_t_const_Vec4f(std::vector<cv::Vec4f>* instance, size_t index, const cv::Vec4f* val) {
			(*instance)[index] = *val;
	}

	// std::vector<cv::Vec4f>::clone() generated
	// ("std::vector<cv::Vec4f>::clone", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Vec4f>* std_vectorLcv_Vec4fG_clone_const(const std::vector<cv::Vec4f>* instance) {
			std::vector<cv::Vec4f> ret = std::vector<cv::Vec4f>(*instance);
			return new std::vector<cv::Vec4f>(ret);
	}

	// std::vector<cv::Vec4f>::data() generated
	// ("std::vector<cv::Vec4f>::data", vec![(pred!(const, [], []), _)]),
	const cv::Vec4f* std_vectorLcv_Vec4fG_data_const(const std::vector<cv::Vec4f>* instance) {
			const cv::Vec4f* ret = instance->data();
			return ret;
	}

	// std::vector<cv::Vec4f>::dataMut() generated
	// ("std::vector<cv::Vec4f>::dataMut", vec![(pred!(mut, [], []), _)]),
	cv::Vec4f* std_vectorLcv_Vec4fG_dataMut(std::vector<cv::Vec4f>* instance) {
			cv::Vec4f* ret = instance->data();
			return ret;
	}

	// cv::fromSlice(SimpleClass, Primitive) generated
	// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const cv::Vec4f*", "size_t"]), _)]),
	std::vector<cv::Vec4f>* cv_fromSlice_const_const_Vec4fX_size_t(const cv::Vec4f* data, size_t len) {
			return new std::vector<cv::Vec4f>(data, data + len);
	}

	// std::vector<cv::Vec4f>::inputArray() generated
	// ("std::vector<cv::Vec4f>::inputArray", vec![(pred!(const, [], []), _)]),
	void std_vectorLcv_Vec4fG_inputArray_const(const std::vector<cv::Vec4f>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			const cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new const cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<cv::Vec4f>::outputArray() generated
	// ("std::vector<cv::Vec4f>::outputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_Vec4fG_outputArray(std::vector<cv::Vec4f>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<cv::Vec4f>::inputOutputArray() generated
	// ("std::vector<cv::Vec4f>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_Vec4fG_inputOutputArray(std::vector<cv::Vec4f>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

}


