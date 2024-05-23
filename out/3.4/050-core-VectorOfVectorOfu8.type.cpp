extern "C" {
	// std::vector<std::vector<unsigned char>>::new() generated
	// ("std::vector<std::vector<unsigned char>>::new", vec![(pred!(const, [], []), _)]),
	std::vector<std::vector<unsigned char>>* std_vectorLstd_vectorLunsigned_charGG_new_const() {
			std::vector<std::vector<unsigned char>>* ret = new std::vector<std::vector<unsigned char>>();
			return ret;
	}

	// std::vector<std::vector<unsigned char>>::delete() generated
	// ("std::vector<std::vector<unsigned char>>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLunsigned_charGG_delete(std::vector<std::vector<unsigned char>>* instance) {
			delete instance;
	}

	// std::vector<std::vector<unsigned char>>::len() generated
	// ("std::vector<std::vector<unsigned char>>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_vectorLunsigned_charGG_len_const(const std::vector<std::vector<unsigned char>>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<std::vector<unsigned char>>::isEmpty() generated
	// ("std::vector<std::vector<unsigned char>>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLstd_vectorLunsigned_charGG_isEmpty_const(const std::vector<std::vector<unsigned char>>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<std::vector<unsigned char>>::capacity() generated
	// ("std::vector<std::vector<unsigned char>>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_vectorLunsigned_charGG_capacity_const(const std::vector<std::vector<unsigned char>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<std::vector<unsigned char>>::shrinkToFit() generated
	// ("std::vector<std::vector<unsigned char>>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLunsigned_charGG_shrinkToFit(std::vector<std::vector<unsigned char>>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<std::vector<unsigned char>>::reserve(Primitive) generated
	// ("std::vector<std::vector<unsigned char>>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLunsigned_charGG_reserve_size_t(std::vector<std::vector<unsigned char>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<std::vector<unsigned char>>::remove(Primitive) generated
	// ("std::vector<std::vector<unsigned char>>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLunsigned_charGG_remove_size_t(std::vector<std::vector<unsigned char>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<std::vector<unsigned char>>::swap(Primitive, Primitive) generated
	// ("std::vector<std::vector<unsigned char>>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLstd_vectorLunsigned_charGG_swap_size_t_size_t(std::vector<std::vector<unsigned char>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<std::vector<unsigned char>>::clear() generated
	// ("std::vector<std::vector<unsigned char>>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLunsigned_charGG_clear(std::vector<std::vector<unsigned char>>* instance) {
			instance->clear();
	}

	// std::vector<std::vector<unsigned char>>::push(CppPassByVoidPtr) generated
	// ("std::vector<std::vector<unsigned char>>::push", vec![(pred!(mut, ["val"], ["const std::vector<unsigned char>"]), _)]),
	void std_vectorLstd_vectorLunsigned_charGG_push_const_vectorLunsigned_charG(std::vector<std::vector<unsigned char>>* instance, const std::vector<unsigned char>* val) {
			instance->push_back(*val);
	}

	// std::vector<std::vector<unsigned char>>::insert(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::vector<unsigned char>>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::vector<unsigned char>"]), _)]),
	void std_vectorLstd_vectorLunsigned_charGG_insert_size_t_const_vectorLunsigned_charG(std::vector<std::vector<unsigned char>>* instance, size_t index, const std::vector<unsigned char>* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<std::vector<unsigned char>>::get(Primitive) generated
	// ("std::vector<std::vector<unsigned char>>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLunsigned_charGG_get_const_size_t(const std::vector<std::vector<unsigned char>>* instance, size_t index, std::vector<unsigned char>** ocvrs_return) {
			std::vector<unsigned char> ret = (*instance)[index];
			*ocvrs_return = new std::vector<unsigned char>(ret);
	}

	// std::vector<std::vector<unsigned char>>::set(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::vector<unsigned char>>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::vector<unsigned char>"]), _)]),
	void std_vectorLstd_vectorLunsigned_charGG_set_size_t_const_vectorLunsigned_charG(std::vector<std::vector<unsigned char>>* instance, size_t index, const std::vector<unsigned char>* val) {
			(*instance)[index] = *val;
	}

	// std::vector<std::vector<unsigned char>>::inputArray() generated
	// ("std::vector<std::vector<unsigned char>>::inputArray", vec![(pred!(const, [], []), _)]),
	void std_vectorLstd_vectorLunsigned_charGG_inputArray_const(const std::vector<std::vector<unsigned char>>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			const cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new const cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<std::vector<unsigned char>>::outputArray() generated
	// ("std::vector<std::vector<unsigned char>>::outputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLunsigned_charGG_outputArray(std::vector<std::vector<unsigned char>>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<std::vector<unsigned char>>::inputOutputArray() generated
	// ("std::vector<std::vector<unsigned char>>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLunsigned_charGG_inputOutputArray(std::vector<std::vector<unsigned char>>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

}


