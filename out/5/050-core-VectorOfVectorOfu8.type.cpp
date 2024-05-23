extern "C" {
	// std::vector<std::vector<uint8_t>>::new() generated
	// ("std::vector<std::vector<uint8_t>>::new", vec![(pred!(const, [], []), _)]),
	std::vector<std::vector<uint8_t>>* std_vectorLstd_vectorLuint8_tGG_new_const() {
			std::vector<std::vector<uint8_t>>* ret = new std::vector<std::vector<uint8_t>>();
			return ret;
	}

	// std::vector<std::vector<uint8_t>>::delete() generated
	// ("std::vector<std::vector<uint8_t>>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLuint8_tGG_delete(std::vector<std::vector<uint8_t>>* instance) {
			delete instance;
	}

	// std::vector<std::vector<uint8_t>>::len() generated
	// ("std::vector<std::vector<uint8_t>>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_vectorLuint8_tGG_len_const(const std::vector<std::vector<uint8_t>>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<std::vector<uint8_t>>::isEmpty() generated
	// ("std::vector<std::vector<uint8_t>>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLstd_vectorLuint8_tGG_isEmpty_const(const std::vector<std::vector<uint8_t>>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<std::vector<uint8_t>>::capacity() generated
	// ("std::vector<std::vector<uint8_t>>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_vectorLuint8_tGG_capacity_const(const std::vector<std::vector<uint8_t>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<std::vector<uint8_t>>::shrinkToFit() generated
	// ("std::vector<std::vector<uint8_t>>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLuint8_tGG_shrinkToFit(std::vector<std::vector<uint8_t>>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<std::vector<uint8_t>>::reserve(Primitive) generated
	// ("std::vector<std::vector<uint8_t>>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLuint8_tGG_reserve_size_t(std::vector<std::vector<uint8_t>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<std::vector<uint8_t>>::remove(Primitive) generated
	// ("std::vector<std::vector<uint8_t>>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLuint8_tGG_remove_size_t(std::vector<std::vector<uint8_t>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<std::vector<uint8_t>>::swap(Primitive, Primitive) generated
	// ("std::vector<std::vector<uint8_t>>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLstd_vectorLuint8_tGG_swap_size_t_size_t(std::vector<std::vector<uint8_t>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<std::vector<uint8_t>>::clear() generated
	// ("std::vector<std::vector<uint8_t>>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLuint8_tGG_clear(std::vector<std::vector<uint8_t>>* instance) {
			instance->clear();
	}

	// std::vector<std::vector<uint8_t>>::push(CppPassByVoidPtr) generated
	// ("std::vector<std::vector<uint8_t>>::push", vec![(pred!(mut, ["val"], ["const std::vector<uint8_t>"]), _)]),
	void std_vectorLstd_vectorLuint8_tGG_push_const_vectorLuint8_tG(std::vector<std::vector<uint8_t>>* instance, const std::vector<uint8_t>* val) {
			instance->push_back(*val);
	}

	// std::vector<std::vector<uint8_t>>::insert(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::vector<uint8_t>>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::vector<uint8_t>"]), _)]),
	void std_vectorLstd_vectorLuint8_tGG_insert_size_t_const_vectorLuint8_tG(std::vector<std::vector<uint8_t>>* instance, size_t index, const std::vector<uint8_t>* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<std::vector<uint8_t>>::get(Primitive) generated
	// ("std::vector<std::vector<uint8_t>>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLuint8_tGG_get_const_size_t(const std::vector<std::vector<uint8_t>>* instance, size_t index, std::vector<uint8_t>** ocvrs_return) {
			std::vector<uint8_t> ret = (*instance)[index];
			*ocvrs_return = new std::vector<uint8_t>(ret);
	}

	// std::vector<std::vector<uint8_t>>::set(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::vector<uint8_t>>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::vector<uint8_t>"]), _)]),
	void std_vectorLstd_vectorLuint8_tGG_set_size_t_const_vectorLuint8_tG(std::vector<std::vector<uint8_t>>* instance, size_t index, const std::vector<uint8_t>* val) {
			(*instance)[index] = *val;
	}

	// std::vector<std::vector<uint8_t>>::inputArray() generated
	// ("std::vector<std::vector<uint8_t>>::inputArray", vec![(pred!(const, [], []), _)]),
	void std_vectorLstd_vectorLuint8_tGG_inputArray_const(const std::vector<std::vector<uint8_t>>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			const cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new const cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<std::vector<uint8_t>>::outputArray() generated
	// ("std::vector<std::vector<uint8_t>>::outputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLuint8_tGG_outputArray(std::vector<std::vector<uint8_t>>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<std::vector<uint8_t>>::inputOutputArray() generated
	// ("std::vector<std::vector<uint8_t>>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLuint8_tGG_inputOutputArray(std::vector<std::vector<uint8_t>>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

}


