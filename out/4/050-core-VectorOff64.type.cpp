extern "C" {
	// std::vector<double>::new() generated
	// ("std::vector<double>::new", vec![(pred!(const, [], []), _)]),
	std::vector<double>* std_vectorLdoubleG_new_const() {
			std::vector<double>* ret = new std::vector<double>();
			return ret;
	}

	// std::vector<double>::delete() generated
	// ("std::vector<double>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLdoubleG_delete(std::vector<double>* instance) {
			delete instance;
	}

	// std::vector<double>::len() generated
	// ("std::vector<double>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLdoubleG_len_const(const std::vector<double>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<double>::isEmpty() generated
	// ("std::vector<double>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLdoubleG_isEmpty_const(const std::vector<double>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<double>::capacity() generated
	// ("std::vector<double>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLdoubleG_capacity_const(const std::vector<double>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<double>::shrinkToFit() generated
	// ("std::vector<double>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLdoubleG_shrinkToFit(std::vector<double>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<double>::reserve(Primitive) generated
	// ("std::vector<double>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLdoubleG_reserve_size_t(std::vector<double>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<double>::remove(Primitive) generated
	// ("std::vector<double>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLdoubleG_remove_size_t(std::vector<double>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<double>::swap(Primitive, Primitive) generated
	// ("std::vector<double>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLdoubleG_swap_size_t_size_t(std::vector<double>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<double>::clear() generated
	// ("std::vector<double>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLdoubleG_clear(std::vector<double>* instance) {
			instance->clear();
	}

	// std::vector<double>::push(Primitive) generated
	// ("std::vector<double>::push", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void std_vectorLdoubleG_push_const_double(std::vector<double>* instance, const double val) {
			instance->push_back(val);
	}

	// std::vector<double>::insert(Primitive, Primitive) generated
	// ("std::vector<double>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const double"]), _)]),
	void std_vectorLdoubleG_insert_size_t_const_double(std::vector<double>* instance, size_t index, const double val) {
			instance->insert(instance->begin() + index, val);
	}

	// std::vector<double>::get(Primitive) generated
	// ("std::vector<double>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLdoubleG_get_const_size_t(const std::vector<double>* instance, size_t index, double* ocvrs_return) {
			double ret = (*instance)[index];
			*ocvrs_return = ret;
	}

	// std::vector<double>::set(Primitive, Primitive) generated
	// ("std::vector<double>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const double"]), _)]),
	void std_vectorLdoubleG_set_size_t_const_double(std::vector<double>* instance, size_t index, const double val) {
			(*instance)[index] = val;
	}

	// std::vector<double>::clone() generated
	// ("std::vector<double>::clone", vec![(pred!(const, [], []), _)]),
	std::vector<double>* std_vectorLdoubleG_clone_const(const std::vector<double>* instance) {
			std::vector<double> ret = std::vector<double>(*instance);
			return new std::vector<double>(ret);
	}

	// std::vector<double>::data() generated
	// ("std::vector<double>::data", vec![(pred!(const, [], []), _)]),
	const double* std_vectorLdoubleG_data_const(const std::vector<double>* instance) {
			const double* ret = instance->data();
			return ret;
	}

	// std::vector<double>::dataMut() generated
	// ("std::vector<double>::dataMut", vec![(pred!(mut, [], []), _)]),
	double* std_vectorLdoubleG_dataMut(std::vector<double>* instance) {
			double* ret = instance->data();
			return ret;
	}

	// cv::fromSlice(Indirect, Primitive) generated
	// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const double*", "size_t"]), _)]),
	std::vector<double>* cv_fromSlice_const_const_doubleX_size_t(const double* data, size_t len) {
			return new std::vector<double>(data, data + len);
	}

	// std::vector<double>::inputArray() generated
	// ("std::vector<double>::inputArray", vec![(pred!(const, [], []), _)]),
	void std_vectorLdoubleG_inputArray_const(const std::vector<double>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			const cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new const cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<double>::outputArray() generated
	// ("std::vector<double>::outputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLdoubleG_outputArray(std::vector<double>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<double>::inputOutputArray() generated
	// ("std::vector<double>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLdoubleG_inputOutputArray(std::vector<double>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

}


