extern "C" {
	// std::vector<std::vector<double>>::new() generated
	// ("std::vector<std::vector<double>>::new", vec![(pred!(const, [], []), _)]),
	std::vector<std::vector<double>>* std_vectorLstd_vectorLdoubleGG_new_const() {
			std::vector<std::vector<double>>* ret = new std::vector<std::vector<double>>();
			return ret;
	}

	// std::vector<std::vector<double>>::delete() generated
	// ("std::vector<std::vector<double>>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLdoubleGG_delete(std::vector<std::vector<double>>* instance) {
			delete instance;
	}

	// std::vector<std::vector<double>>::len() generated
	// ("std::vector<std::vector<double>>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_vectorLdoubleGG_len_const(const std::vector<std::vector<double>>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<std::vector<double>>::isEmpty() generated
	// ("std::vector<std::vector<double>>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLstd_vectorLdoubleGG_isEmpty_const(const std::vector<std::vector<double>>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<std::vector<double>>::capacity() generated
	// ("std::vector<std::vector<double>>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_vectorLdoubleGG_capacity_const(const std::vector<std::vector<double>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<std::vector<double>>::shrinkToFit() generated
	// ("std::vector<std::vector<double>>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLdoubleGG_shrinkToFit(std::vector<std::vector<double>>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<std::vector<double>>::reserve(Primitive) generated
	// ("std::vector<std::vector<double>>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLdoubleGG_reserve_size_t(std::vector<std::vector<double>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<std::vector<double>>::remove(Primitive) generated
	// ("std::vector<std::vector<double>>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLdoubleGG_remove_size_t(std::vector<std::vector<double>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<std::vector<double>>::swap(Primitive, Primitive) generated
	// ("std::vector<std::vector<double>>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLstd_vectorLdoubleGG_swap_size_t_size_t(std::vector<std::vector<double>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<std::vector<double>>::clear() generated
	// ("std::vector<std::vector<double>>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLdoubleGG_clear(std::vector<std::vector<double>>* instance) {
			instance->clear();
	}

	// std::vector<std::vector<double>>::push(CppPassByVoidPtr) generated
	// ("std::vector<std::vector<double>>::push", vec![(pred!(mut, ["val"], ["const std::vector<double>"]), _)]),
	void std_vectorLstd_vectorLdoubleGG_push_const_vectorLdoubleG(std::vector<std::vector<double>>* instance, const std::vector<double>* val) {
			instance->push_back(*val);
	}

	// std::vector<std::vector<double>>::insert(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::vector<double>>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::vector<double>"]), _)]),
	void std_vectorLstd_vectorLdoubleGG_insert_size_t_const_vectorLdoubleG(std::vector<std::vector<double>>* instance, size_t index, const std::vector<double>* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<std::vector<double>>::get(Primitive) generated
	// ("std::vector<std::vector<double>>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLdoubleGG_get_const_size_t(const std::vector<std::vector<double>>* instance, size_t index, std::vector<double>** ocvrs_return) {
			std::vector<double> ret = (*instance)[index];
			*ocvrs_return = new std::vector<double>(ret);
	}

	// std::vector<std::vector<double>>::set(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::vector<double>>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::vector<double>"]), _)]),
	void std_vectorLstd_vectorLdoubleGG_set_size_t_const_vectorLdoubleG(std::vector<std::vector<double>>* instance, size_t index, const std::vector<double>* val) {
			(*instance)[index] = *val;
	}

	// std::vector<std::vector<double>>::inputArray() generated
	// ("std::vector<std::vector<double>>::inputArray", vec![(pred!(const, [], []), _)]),
	void std_vectorLstd_vectorLdoubleGG_inputArray_const(const std::vector<std::vector<double>>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			const cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new const cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<std::vector<double>>::outputArray() generated
	// ("std::vector<std::vector<double>>::outputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLdoubleGG_outputArray(std::vector<std::vector<double>>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<std::vector<double>>::inputOutputArray() generated
	// ("std::vector<std::vector<double>>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLdoubleGG_inputOutputArray(std::vector<std::vector<double>>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

}


