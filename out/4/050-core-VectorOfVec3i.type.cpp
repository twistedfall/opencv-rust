extern "C" {
	// std::vector<cv::Vec3i>::new() generated
	// ("std::vector<cv::Vec3i>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Vec3i>* std_vectorLcv_Vec3iG_new_const() {
			std::vector<cv::Vec3i>* ret = new std::vector<cv::Vec3i>();
			return ret;
	}

	// std::vector<cv::Vec3i>::delete() generated
	// ("std::vector<cv::Vec3i>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_Vec3iG_delete(std::vector<cv::Vec3i>* instance) {
			delete instance;
	}

	// std::vector<cv::Vec3i>::len() generated
	// ("std::vector<cv::Vec3i>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_Vec3iG_len_const(const std::vector<cv::Vec3i>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::Vec3i>::isEmpty() generated
	// ("std::vector<cv::Vec3i>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_Vec3iG_isEmpty_const(const std::vector<cv::Vec3i>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::Vec3i>::capacity() generated
	// ("std::vector<cv::Vec3i>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_Vec3iG_capacity_const(const std::vector<cv::Vec3i>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::Vec3i>::shrinkToFit() generated
	// ("std::vector<cv::Vec3i>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_Vec3iG_shrinkToFit(std::vector<cv::Vec3i>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::Vec3i>::reserve(Primitive) generated
	// ("std::vector<cv::Vec3i>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_Vec3iG_reserve_size_t(std::vector<cv::Vec3i>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::Vec3i>::remove(Primitive) generated
	// ("std::vector<cv::Vec3i>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_Vec3iG_remove_size_t(std::vector<cv::Vec3i>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::Vec3i>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::Vec3i>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_Vec3iG_swap_size_t_size_t(std::vector<cv::Vec3i>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::Vec3i>::clear() generated
	// ("std::vector<cv::Vec3i>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_Vec3iG_clear(std::vector<cv::Vec3i>* instance) {
			instance->clear();
	}

	// std::vector<cv::Vec3i>::push(SimpleClass) generated
	// ("std::vector<cv::Vec3i>::push", vec![(pred!(mut, ["val"], ["const cv::Vec3i"]), _)]),
	void std_vectorLcv_Vec3iG_push_const_Vec3i(std::vector<cv::Vec3i>* instance, const cv::Vec3i* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::Vec3i>::insert(Primitive, SimpleClass) generated
	// ("std::vector<cv::Vec3i>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Vec3i"]), _)]),
	void std_vectorLcv_Vec3iG_insert_size_t_const_Vec3i(std::vector<cv::Vec3i>* instance, size_t index, const cv::Vec3i* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::Vec3i>::get(Primitive) generated
	// ("std::vector<cv::Vec3i>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_Vec3iG_get_const_size_t(const std::vector<cv::Vec3i>* instance, size_t index, cv::Vec3i* ocvrs_return) {
			cv::Vec3i ret = (*instance)[index];
			*ocvrs_return = ret;
	}

	// std::vector<cv::Vec3i>::set(Primitive, SimpleClass) generated
	// ("std::vector<cv::Vec3i>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Vec3i"]), _)]),
	void std_vectorLcv_Vec3iG_set_size_t_const_Vec3i(std::vector<cv::Vec3i>* instance, size_t index, const cv::Vec3i* val) {
			(*instance)[index] = *val;
	}

	// std::vector<cv::Vec3i>::clone() generated
	// ("std::vector<cv::Vec3i>::clone", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Vec3i>* std_vectorLcv_Vec3iG_clone_const(const std::vector<cv::Vec3i>* instance) {
			std::vector<cv::Vec3i> ret = std::vector<cv::Vec3i>(*instance);
			return new std::vector<cv::Vec3i>(ret);
	}

	// std::vector<cv::Vec3i>::data() generated
	// ("std::vector<cv::Vec3i>::data", vec![(pred!(const, [], []), _)]),
	const cv::Vec3i* std_vectorLcv_Vec3iG_data_const(const std::vector<cv::Vec3i>* instance) {
			const cv::Vec3i* ret = instance->data();
			return ret;
	}

	// std::vector<cv::Vec3i>::dataMut() generated
	// ("std::vector<cv::Vec3i>::dataMut", vec![(pred!(mut, [], []), _)]),
	cv::Vec3i* std_vectorLcv_Vec3iG_dataMut(std::vector<cv::Vec3i>* instance) {
			cv::Vec3i* ret = instance->data();
			return ret;
	}

	// cv::fromSlice(SimpleClass, Primitive) generated
	// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const cv::Vec3i*", "size_t"]), _)]),
	std::vector<cv::Vec3i>* cv_fromSlice_const_const_Vec3iX_size_t(const cv::Vec3i* data, size_t len) {
			return new std::vector<cv::Vec3i>(data, data + len);
	}

	// std::vector<cv::Vec3i>::inputArray() generated
	// ("std::vector<cv::Vec3i>::inputArray", vec![(pred!(const, [], []), _)]),
	void std_vectorLcv_Vec3iG_inputArray_const(const std::vector<cv::Vec3i>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			const cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new const cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<cv::Vec3i>::outputArray() generated
	// ("std::vector<cv::Vec3i>::outputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_Vec3iG_outputArray(std::vector<cv::Vec3i>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<cv::Vec3i>::inputOutputArray() generated
	// ("std::vector<cv::Vec3i>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_Vec3iG_inputOutputArray(std::vector<cv::Vec3i>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

}


