extern "C" {
	// std::vector<cv::Vec2f>::new() generated
	// ("std::vector<cv::Vec2f>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Vec2f>* std_vectorLcv_Vec2fG_new_const() {
			std::vector<cv::Vec2f>* ret = new std::vector<cv::Vec2f>();
			return ret;
	}

	// std::vector<cv::Vec2f>::delete() generated
	// ("std::vector<cv::Vec2f>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_Vec2fG_delete(std::vector<cv::Vec2f>* instance) {
			delete instance;
	}

	// std::vector<cv::Vec2f>::len() generated
	// ("std::vector<cv::Vec2f>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_Vec2fG_len_const(const std::vector<cv::Vec2f>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::Vec2f>::isEmpty() generated
	// ("std::vector<cv::Vec2f>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_Vec2fG_isEmpty_const(const std::vector<cv::Vec2f>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::Vec2f>::capacity() generated
	// ("std::vector<cv::Vec2f>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_Vec2fG_capacity_const(const std::vector<cv::Vec2f>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::Vec2f>::shrinkToFit() generated
	// ("std::vector<cv::Vec2f>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_Vec2fG_shrinkToFit(std::vector<cv::Vec2f>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::Vec2f>::reserve(Primitive) generated
	// ("std::vector<cv::Vec2f>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_Vec2fG_reserve_size_t(std::vector<cv::Vec2f>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::Vec2f>::remove(Primitive) generated
	// ("std::vector<cv::Vec2f>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_Vec2fG_remove_size_t(std::vector<cv::Vec2f>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::Vec2f>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::Vec2f>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_Vec2fG_swap_size_t_size_t(std::vector<cv::Vec2f>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::Vec2f>::clear() generated
	// ("std::vector<cv::Vec2f>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_Vec2fG_clear(std::vector<cv::Vec2f>* instance) {
			instance->clear();
	}

	// std::vector<cv::Vec2f>::push(SimpleClass) generated
	// ("std::vector<cv::Vec2f>::push", vec![(pred!(mut, ["val"], ["const cv::Vec2f"]), _)]),
	void std_vectorLcv_Vec2fG_push_const_Vec2f(std::vector<cv::Vec2f>* instance, const cv::Vec2f* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::Vec2f>::insert(Primitive, SimpleClass) generated
	// ("std::vector<cv::Vec2f>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Vec2f"]), _)]),
	void std_vectorLcv_Vec2fG_insert_size_t_const_Vec2f(std::vector<cv::Vec2f>* instance, size_t index, const cv::Vec2f* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::Vec2f>::get(Primitive) generated
	// ("std::vector<cv::Vec2f>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_Vec2fG_get_const_size_t(const std::vector<cv::Vec2f>* instance, size_t index, cv::Vec2f* ocvrs_return) {
			cv::Vec2f ret = (*instance)[index];
			*ocvrs_return = ret;
	}

	// std::vector<cv::Vec2f>::set(Primitive, SimpleClass) generated
	// ("std::vector<cv::Vec2f>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Vec2f"]), _)]),
	void std_vectorLcv_Vec2fG_set_size_t_const_Vec2f(std::vector<cv::Vec2f>* instance, size_t index, const cv::Vec2f* val) {
			(*instance)[index] = *val;
	}

	// std::vector<cv::Vec2f>::clone() generated
	// ("std::vector<cv::Vec2f>::clone", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Vec2f>* std_vectorLcv_Vec2fG_clone_const(const std::vector<cv::Vec2f>* instance) {
			std::vector<cv::Vec2f> ret = std::vector<cv::Vec2f>(*instance);
			return new std::vector<cv::Vec2f>(ret);
	}

	// std::vector<cv::Vec2f>::data() generated
	// ("std::vector<cv::Vec2f>::data", vec![(pred!(const, [], []), _)]),
	const cv::Vec2f* std_vectorLcv_Vec2fG_data_const(const std::vector<cv::Vec2f>* instance) {
			const cv::Vec2f* ret = instance->data();
			return ret;
	}

	// std::vector<cv::Vec2f>::dataMut() generated
	// ("std::vector<cv::Vec2f>::dataMut", vec![(pred!(mut, [], []), _)]),
	cv::Vec2f* std_vectorLcv_Vec2fG_dataMut(std::vector<cv::Vec2f>* instance) {
			cv::Vec2f* ret = instance->data();
			return ret;
	}

	// cv::fromSlice(SimpleClass, Primitive) generated
	// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const cv::Vec2f*", "size_t"]), _)]),
	std::vector<cv::Vec2f>* cv_fromSlice_const_const_Vec2fX_size_t(const cv::Vec2f* data, size_t len) {
			return new std::vector<cv::Vec2f>(data, data + len);
	}

	// std::vector<cv::Vec2f>::inputArray() generated
	// ("std::vector<cv::Vec2f>::inputArray", vec![(pred!(const, [], []), _)]),
	void std_vectorLcv_Vec2fG_inputArray_const(const std::vector<cv::Vec2f>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			const cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new const cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<cv::Vec2f>::outputArray() generated
	// ("std::vector<cv::Vec2f>::outputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_Vec2fG_outputArray(std::vector<cv::Vec2f>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<cv::Vec2f>::inputOutputArray() generated
	// ("std::vector<cv::Vec2f>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_Vec2fG_inputOutputArray(std::vector<cv::Vec2f>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

}


