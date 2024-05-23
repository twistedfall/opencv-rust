extern "C" {
	// std::vector<std::vector<int>>::new() generated
	// ("std::vector<std::vector<int>>::new", vec![(pred!(const, [], []), _)]),
	std::vector<std::vector<int>>* std_vectorLstd_vectorLintGG_new_const() {
			std::vector<std::vector<int>>* ret = new std::vector<std::vector<int>>();
			return ret;
	}

	// std::vector<std::vector<int>>::delete() generated
	// ("std::vector<std::vector<int>>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLintGG_delete(std::vector<std::vector<int>>* instance) {
			delete instance;
	}

	// std::vector<std::vector<int>>::len() generated
	// ("std::vector<std::vector<int>>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_vectorLintGG_len_const(const std::vector<std::vector<int>>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<std::vector<int>>::isEmpty() generated
	// ("std::vector<std::vector<int>>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLstd_vectorLintGG_isEmpty_const(const std::vector<std::vector<int>>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<std::vector<int>>::capacity() generated
	// ("std::vector<std::vector<int>>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_vectorLintGG_capacity_const(const std::vector<std::vector<int>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<std::vector<int>>::shrinkToFit() generated
	// ("std::vector<std::vector<int>>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLintGG_shrinkToFit(std::vector<std::vector<int>>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<std::vector<int>>::reserve(Primitive) generated
	// ("std::vector<std::vector<int>>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLintGG_reserve_size_t(std::vector<std::vector<int>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<std::vector<int>>::remove(Primitive) generated
	// ("std::vector<std::vector<int>>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLintGG_remove_size_t(std::vector<std::vector<int>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<std::vector<int>>::swap(Primitive, Primitive) generated
	// ("std::vector<std::vector<int>>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLstd_vectorLintGG_swap_size_t_size_t(std::vector<std::vector<int>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<std::vector<int>>::clear() generated
	// ("std::vector<std::vector<int>>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLintGG_clear(std::vector<std::vector<int>>* instance) {
			instance->clear();
	}

	// std::vector<std::vector<int>>::push(CppPassByVoidPtr) generated
	// ("std::vector<std::vector<int>>::push", vec![(pred!(mut, ["val"], ["const std::vector<int>"]), _)]),
	void std_vectorLstd_vectorLintGG_push_const_vectorLintG(std::vector<std::vector<int>>* instance, const std::vector<int>* val) {
			instance->push_back(*val);
	}

	// std::vector<std::vector<int>>::insert(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::vector<int>>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::vector<int>"]), _)]),
	void std_vectorLstd_vectorLintGG_insert_size_t_const_vectorLintG(std::vector<std::vector<int>>* instance, size_t index, const std::vector<int>* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<std::vector<int>>::get(Primitive) generated
	// ("std::vector<std::vector<int>>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLintGG_get_const_size_t(const std::vector<std::vector<int>>* instance, size_t index, std::vector<int>** ocvrs_return) {
			std::vector<int> ret = (*instance)[index];
			*ocvrs_return = new std::vector<int>(ret);
	}

	// std::vector<std::vector<int>>::set(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::vector<int>>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::vector<int>"]), _)]),
	void std_vectorLstd_vectorLintGG_set_size_t_const_vectorLintG(std::vector<std::vector<int>>* instance, size_t index, const std::vector<int>* val) {
			(*instance)[index] = *val;
	}

	// std::vector<std::vector<int>>::inputArray() generated
	// ("std::vector<std::vector<int>>::inputArray", vec![(pred!(const, [], []), _)]),
	void std_vectorLstd_vectorLintGG_inputArray_const(const std::vector<std::vector<int>>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			const cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new const cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<std::vector<int>>::outputArray() generated
	// ("std::vector<std::vector<int>>::outputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLintGG_outputArray(std::vector<std::vector<int>>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<std::vector<int>>::inputOutputArray() generated
	// ("std::vector<std::vector<int>>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLintGG_inputOutputArray(std::vector<std::vector<int>>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

}


