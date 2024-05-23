extern "C" {
	// std::vector<int>::new() generated
	// ("std::vector<int>::new", vec![(pred!(const, [], []), _)]),
	std::vector<int>* std_vectorLintG_new_const() {
			std::vector<int>* ret = new std::vector<int>();
			return ret;
	}

	// std::vector<int>::delete() generated
	// ("std::vector<int>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLintG_delete(std::vector<int>* instance) {
			delete instance;
	}

	// std::vector<int>::len() generated
	// ("std::vector<int>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLintG_len_const(const std::vector<int>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<int>::isEmpty() generated
	// ("std::vector<int>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLintG_isEmpty_const(const std::vector<int>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<int>::capacity() generated
	// ("std::vector<int>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLintG_capacity_const(const std::vector<int>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<int>::shrinkToFit() generated
	// ("std::vector<int>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLintG_shrinkToFit(std::vector<int>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<int>::reserve(Primitive) generated
	// ("std::vector<int>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLintG_reserve_size_t(std::vector<int>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<int>::remove(Primitive) generated
	// ("std::vector<int>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLintG_remove_size_t(std::vector<int>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<int>::swap(Primitive, Primitive) generated
	// ("std::vector<int>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLintG_swap_size_t_size_t(std::vector<int>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<int>::clear() generated
	// ("std::vector<int>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLintG_clear(std::vector<int>* instance) {
			instance->clear();
	}

	// std::vector<int>::push(Primitive) generated
	// ("std::vector<int>::push", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void std_vectorLintG_push_const_int(std::vector<int>* instance, const int val) {
			instance->push_back(val);
	}

	// std::vector<int>::insert(Primitive, Primitive) generated
	// ("std::vector<int>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const int"]), _)]),
	void std_vectorLintG_insert_size_t_const_int(std::vector<int>* instance, size_t index, const int val) {
			instance->insert(instance->begin() + index, val);
	}

	// std::vector<int>::get(Primitive) generated
	// ("std::vector<int>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLintG_get_const_size_t(const std::vector<int>* instance, size_t index, int* ocvrs_return) {
			int ret = (*instance)[index];
			*ocvrs_return = ret;
	}

	// std::vector<int>::set(Primitive, Primitive) generated
	// ("std::vector<int>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const int"]), _)]),
	void std_vectorLintG_set_size_t_const_int(std::vector<int>* instance, size_t index, const int val) {
			(*instance)[index] = val;
	}

	// std::vector<int>::clone() generated
	// ("std::vector<int>::clone", vec![(pred!(const, [], []), _)]),
	std::vector<int>* std_vectorLintG_clone_const(const std::vector<int>* instance) {
			std::vector<int> ret = std::vector<int>(*instance);
			return new std::vector<int>(ret);
	}

	// std::vector<int>::data() generated
	// ("std::vector<int>::data", vec![(pred!(const, [], []), _)]),
	const int* std_vectorLintG_data_const(const std::vector<int>* instance) {
			const int* ret = instance->data();
			return ret;
	}

	// std::vector<int>::dataMut() generated
	// ("std::vector<int>::dataMut", vec![(pred!(mut, [], []), _)]),
	int* std_vectorLintG_dataMut(std::vector<int>* instance) {
			int* ret = instance->data();
			return ret;
	}

	// cv::fromSlice(Indirect, Primitive) generated
	// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const int*", "size_t"]), _)]),
	std::vector<int>* cv_fromSlice_const_const_intX_size_t(const int* data, size_t len) {
			return new std::vector<int>(data, data + len);
	}

	// std::vector<int>::inputArray() generated
	// ("std::vector<int>::inputArray", vec![(pred!(const, [], []), _)]),
	void std_vectorLintG_inputArray_const(const std::vector<int>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			const cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new const cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<int>::outputArray() generated
	// ("std::vector<int>::outputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLintG_outputArray(std::vector<int>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<int>::inputOutputArray() generated
	// ("std::vector<int>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLintG_inputOutputArray(std::vector<int>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

}


