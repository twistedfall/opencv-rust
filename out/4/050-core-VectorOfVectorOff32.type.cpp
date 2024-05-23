extern "C" {
	// std::vector<std::vector<float>>::new() generated
	// ("std::vector<std::vector<float>>::new", vec![(pred!(const, [], []), _)]),
	std::vector<std::vector<float>>* std_vectorLstd_vectorLfloatGG_new_const() {
			std::vector<std::vector<float>>* ret = new std::vector<std::vector<float>>();
			return ret;
	}

	// std::vector<std::vector<float>>::delete() generated
	// ("std::vector<std::vector<float>>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLfloatGG_delete(std::vector<std::vector<float>>* instance) {
			delete instance;
	}

	// std::vector<std::vector<float>>::len() generated
	// ("std::vector<std::vector<float>>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_vectorLfloatGG_len_const(const std::vector<std::vector<float>>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<std::vector<float>>::isEmpty() generated
	// ("std::vector<std::vector<float>>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLstd_vectorLfloatGG_isEmpty_const(const std::vector<std::vector<float>>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<std::vector<float>>::capacity() generated
	// ("std::vector<std::vector<float>>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_vectorLfloatGG_capacity_const(const std::vector<std::vector<float>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<std::vector<float>>::shrinkToFit() generated
	// ("std::vector<std::vector<float>>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLfloatGG_shrinkToFit(std::vector<std::vector<float>>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<std::vector<float>>::reserve(Primitive) generated
	// ("std::vector<std::vector<float>>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLfloatGG_reserve_size_t(std::vector<std::vector<float>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<std::vector<float>>::remove(Primitive) generated
	// ("std::vector<std::vector<float>>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLfloatGG_remove_size_t(std::vector<std::vector<float>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<std::vector<float>>::swap(Primitive, Primitive) generated
	// ("std::vector<std::vector<float>>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLstd_vectorLfloatGG_swap_size_t_size_t(std::vector<std::vector<float>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<std::vector<float>>::clear() generated
	// ("std::vector<std::vector<float>>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLfloatGG_clear(std::vector<std::vector<float>>* instance) {
			instance->clear();
	}

	// std::vector<std::vector<float>>::push(CppPassByVoidPtr) generated
	// ("std::vector<std::vector<float>>::push", vec![(pred!(mut, ["val"], ["const std::vector<float>"]), _)]),
	void std_vectorLstd_vectorLfloatGG_push_const_vectorLfloatG(std::vector<std::vector<float>>* instance, const std::vector<float>* val) {
			instance->push_back(*val);
	}

	// std::vector<std::vector<float>>::insert(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::vector<float>>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::vector<float>"]), _)]),
	void std_vectorLstd_vectorLfloatGG_insert_size_t_const_vectorLfloatG(std::vector<std::vector<float>>* instance, size_t index, const std::vector<float>* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<std::vector<float>>::get(Primitive) generated
	// ("std::vector<std::vector<float>>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLfloatGG_get_const_size_t(const std::vector<std::vector<float>>* instance, size_t index, std::vector<float>** ocvrs_return) {
			std::vector<float> ret = (*instance)[index];
			*ocvrs_return = new std::vector<float>(ret);
	}

	// std::vector<std::vector<float>>::set(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::vector<float>>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::vector<float>"]), _)]),
	void std_vectorLstd_vectorLfloatGG_set_size_t_const_vectorLfloatG(std::vector<std::vector<float>>* instance, size_t index, const std::vector<float>* val) {
			(*instance)[index] = *val;
	}

	// std::vector<std::vector<float>>::inputArray() generated
	// ("std::vector<std::vector<float>>::inputArray", vec![(pred!(const, [], []), _)]),
	void std_vectorLstd_vectorLfloatGG_inputArray_const(const std::vector<std::vector<float>>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			const cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new const cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<std::vector<float>>::outputArray() generated
	// ("std::vector<std::vector<float>>::outputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLfloatGG_outputArray(std::vector<std::vector<float>>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<std::vector<float>>::inputOutputArray() generated
	// ("std::vector<std::vector<float>>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLfloatGG_inputOutputArray(std::vector<std::vector<float>>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

}


