extern "C" {
	// std::vector<std::vector<bool>>::new() generated
	// ("std::vector<std::vector<bool>>::new", vec![(pred!(const, [], []), _)]),
	std::vector<std::vector<bool>>* std_vectorLstd_vectorLboolGG_new_const() {
			std::vector<std::vector<bool>>* ret = new std::vector<std::vector<bool>>();
			return ret;
	}

	// std::vector<std::vector<bool>>::delete() generated
	// ("std::vector<std::vector<bool>>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLboolGG_delete(std::vector<std::vector<bool>>* instance) {
			delete instance;
	}

	// std::vector<std::vector<bool>>::len() generated
	// ("std::vector<std::vector<bool>>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_vectorLboolGG_len_const(const std::vector<std::vector<bool>>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<std::vector<bool>>::isEmpty() generated
	// ("std::vector<std::vector<bool>>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLstd_vectorLboolGG_isEmpty_const(const std::vector<std::vector<bool>>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<std::vector<bool>>::capacity() generated
	// ("std::vector<std::vector<bool>>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_vectorLboolGG_capacity_const(const std::vector<std::vector<bool>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<std::vector<bool>>::shrinkToFit() generated
	// ("std::vector<std::vector<bool>>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLboolGG_shrinkToFit(std::vector<std::vector<bool>>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<std::vector<bool>>::reserve(Primitive) generated
	// ("std::vector<std::vector<bool>>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLboolGG_reserve_size_t(std::vector<std::vector<bool>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<std::vector<bool>>::remove(Primitive) generated
	// ("std::vector<std::vector<bool>>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLboolGG_remove_size_t(std::vector<std::vector<bool>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<std::vector<bool>>::swap(Primitive, Primitive) generated
	// ("std::vector<std::vector<bool>>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLstd_vectorLboolGG_swap_size_t_size_t(std::vector<std::vector<bool>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<std::vector<bool>>::clear() generated
	// ("std::vector<std::vector<bool>>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLboolGG_clear(std::vector<std::vector<bool>>* instance) {
			instance->clear();
	}

	// std::vector<std::vector<bool>>::push(CppPassByVoidPtr) generated
	// ("std::vector<std::vector<bool>>::push", vec![(pred!(mut, ["val"], ["const std::vector<bool>"]), _)]),
	void std_vectorLstd_vectorLboolGG_push_const_vectorLboolG(std::vector<std::vector<bool>>* instance, const std::vector<bool>* val) {
			instance->push_back(*val);
	}

	// std::vector<std::vector<bool>>::insert(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::vector<bool>>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::vector<bool>"]), _)]),
	void std_vectorLstd_vectorLboolGG_insert_size_t_const_vectorLboolG(std::vector<std::vector<bool>>* instance, size_t index, const std::vector<bool>* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<std::vector<bool>>::get(Primitive) generated
	// ("std::vector<std::vector<bool>>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLboolGG_get_const_size_t(const std::vector<std::vector<bool>>* instance, size_t index, std::vector<bool>** ocvrs_return) {
			std::vector<bool> ret = (*instance)[index];
			*ocvrs_return = new std::vector<bool>(ret);
	}

	// std::vector<std::vector<bool>>::set(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::vector<bool>>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::vector<bool>"]), _)]),
	void std_vectorLstd_vectorLboolGG_set_size_t_const_vectorLboolG(std::vector<std::vector<bool>>* instance, size_t index, const std::vector<bool>* val) {
			(*instance)[index] = *val;
	}

	#if CV_VERSION_MAJOR == 5
	// std::vector<std::vector<bool>>::inputArray() generated
	// ("std::vector<std::vector<bool>>::inputArray", vec![(pred!(const, [], []), _)]),
	void std_vectorLstd_vectorLboolGG_inputArray_const(const std::vector<std::vector<bool>>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			const cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new const cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	#endif

	#if CV_VERSION_MAJOR == 5
	// std::vector<std::vector<bool>>::outputArray() generated
	// ("std::vector<std::vector<bool>>::outputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLboolGG_outputArray(std::vector<std::vector<bool>>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	#endif

	#if CV_VERSION_MAJOR == 5
	// std::vector<std::vector<bool>>::inputOutputArray() generated
	// ("std::vector<std::vector<bool>>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLboolGG_inputOutputArray(std::vector<std::vector<bool>>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	#endif

}


