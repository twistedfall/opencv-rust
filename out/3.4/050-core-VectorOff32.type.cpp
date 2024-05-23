extern "C" {
	// std::vector<float>::new() generated
	// ("std::vector<float>::new", vec![(pred!(const, [], []), _)]),
	std::vector<float>* std_vectorLfloatG_new_const() {
			std::vector<float>* ret = new std::vector<float>();
			return ret;
	}

	// std::vector<float>::delete() generated
	// ("std::vector<float>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLfloatG_delete(std::vector<float>* instance) {
			delete instance;
	}

	// std::vector<float>::len() generated
	// ("std::vector<float>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLfloatG_len_const(const std::vector<float>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<float>::isEmpty() generated
	// ("std::vector<float>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLfloatG_isEmpty_const(const std::vector<float>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<float>::capacity() generated
	// ("std::vector<float>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLfloatG_capacity_const(const std::vector<float>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<float>::shrinkToFit() generated
	// ("std::vector<float>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLfloatG_shrinkToFit(std::vector<float>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<float>::reserve(Primitive) generated
	// ("std::vector<float>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLfloatG_reserve_size_t(std::vector<float>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<float>::remove(Primitive) generated
	// ("std::vector<float>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLfloatG_remove_size_t(std::vector<float>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<float>::swap(Primitive, Primitive) generated
	// ("std::vector<float>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLfloatG_swap_size_t_size_t(std::vector<float>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<float>::clear() generated
	// ("std::vector<float>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLfloatG_clear(std::vector<float>* instance) {
			instance->clear();
	}

	// std::vector<float>::push(Primitive) generated
	// ("std::vector<float>::push", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void std_vectorLfloatG_push_const_float(std::vector<float>* instance, const float val) {
			instance->push_back(val);
	}

	// std::vector<float>::insert(Primitive, Primitive) generated
	// ("std::vector<float>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const float"]), _)]),
	void std_vectorLfloatG_insert_size_t_const_float(std::vector<float>* instance, size_t index, const float val) {
			instance->insert(instance->begin() + index, val);
	}

	// std::vector<float>::get(Primitive) generated
	// ("std::vector<float>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLfloatG_get_const_size_t(const std::vector<float>* instance, size_t index, float* ocvrs_return) {
			float ret = (*instance)[index];
			*ocvrs_return = ret;
	}

	// std::vector<float>::set(Primitive, Primitive) generated
	// ("std::vector<float>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const float"]), _)]),
	void std_vectorLfloatG_set_size_t_const_float(std::vector<float>* instance, size_t index, const float val) {
			(*instance)[index] = val;
	}

	// std::vector<float>::clone() generated
	// ("std::vector<float>::clone", vec![(pred!(const, [], []), _)]),
	std::vector<float>* std_vectorLfloatG_clone_const(const std::vector<float>* instance) {
			std::vector<float> ret = std::vector<float>(*instance);
			return new std::vector<float>(ret);
	}

	// std::vector<float>::data() generated
	// ("std::vector<float>::data", vec![(pred!(const, [], []), _)]),
	const float* std_vectorLfloatG_data_const(const std::vector<float>* instance) {
			const float* ret = instance->data();
			return ret;
	}

	// std::vector<float>::dataMut() generated
	// ("std::vector<float>::dataMut", vec![(pred!(mut, [], []), _)]),
	float* std_vectorLfloatG_dataMut(std::vector<float>* instance) {
			float* ret = instance->data();
			return ret;
	}

	// cv::fromSlice(Indirect, Primitive) generated
	// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const float*", "size_t"]), _)]),
	std::vector<float>* cv_fromSlice_const_const_floatX_size_t(const float* data, size_t len) {
			return new std::vector<float>(data, data + len);
	}

	// std::vector<float>::inputArray() generated
	// ("std::vector<float>::inputArray", vec![(pred!(const, [], []), _)]),
	void std_vectorLfloatG_inputArray_const(const std::vector<float>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			const cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new const cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<float>::outputArray() generated
	// ("std::vector<float>::outputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLfloatG_outputArray(std::vector<float>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<float>::inputOutputArray() generated
	// ("std::vector<float>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLfloatG_inputOutputArray(std::vector<float>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

}


