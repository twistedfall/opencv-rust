extern "C" {
	// std::vector<unsigned char>::new() generated
	// ("std::vector<unsigned char>::new", vec![(pred!(const, [], []), _)]),
	std::vector<unsigned char>* std_vectorLunsigned_charG_new_const() {
			std::vector<unsigned char>* ret = new std::vector<unsigned char>();
			return ret;
	}

	// std::vector<unsigned char>::delete() generated
	// ("std::vector<unsigned char>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLunsigned_charG_delete(std::vector<unsigned char>* instance) {
			delete instance;
	}

	// std::vector<unsigned char>::len() generated
	// ("std::vector<unsigned char>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLunsigned_charG_len_const(const std::vector<unsigned char>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<unsigned char>::isEmpty() generated
	// ("std::vector<unsigned char>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLunsigned_charG_isEmpty_const(const std::vector<unsigned char>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<unsigned char>::capacity() generated
	// ("std::vector<unsigned char>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLunsigned_charG_capacity_const(const std::vector<unsigned char>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<unsigned char>::shrinkToFit() generated
	// ("std::vector<unsigned char>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLunsigned_charG_shrinkToFit(std::vector<unsigned char>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<unsigned char>::reserve(Primitive) generated
	// ("std::vector<unsigned char>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLunsigned_charG_reserve_size_t(std::vector<unsigned char>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<unsigned char>::remove(Primitive) generated
	// ("std::vector<unsigned char>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLunsigned_charG_remove_size_t(std::vector<unsigned char>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<unsigned char>::swap(Primitive, Primitive) generated
	// ("std::vector<unsigned char>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLunsigned_charG_swap_size_t_size_t(std::vector<unsigned char>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<unsigned char>::clear() generated
	// ("std::vector<unsigned char>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLunsigned_charG_clear(std::vector<unsigned char>* instance) {
			instance->clear();
	}

	// std::vector<unsigned char>::push(Primitive) generated
	// ("std::vector<unsigned char>::push", vec![(pred!(mut, ["val"], ["const unsigned char"]), _)]),
	void std_vectorLunsigned_charG_push_const_unsigned_char(std::vector<unsigned char>* instance, const unsigned char val) {
			instance->push_back(val);
	}

	// std::vector<unsigned char>::insert(Primitive, Primitive) generated
	// ("std::vector<unsigned char>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const unsigned char"]), _)]),
	void std_vectorLunsigned_charG_insert_size_t_const_unsigned_char(std::vector<unsigned char>* instance, size_t index, const unsigned char val) {
			instance->insert(instance->begin() + index, val);
	}

	// std::vector<unsigned char>::get(Primitive) generated
	// ("std::vector<unsigned char>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLunsigned_charG_get_const_size_t(const std::vector<unsigned char>* instance, size_t index, unsigned char* ocvrs_return) {
			unsigned char ret = (*instance)[index];
			*ocvrs_return = ret;
	}

	// std::vector<unsigned char>::set(Primitive, Primitive) generated
	// ("std::vector<unsigned char>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const unsigned char"]), _)]),
	void std_vectorLunsigned_charG_set_size_t_const_unsigned_char(std::vector<unsigned char>* instance, size_t index, const unsigned char val) {
			(*instance)[index] = val;
	}

	// std::vector<unsigned char>::clone() generated
	// ("std::vector<unsigned char>::clone", vec![(pred!(const, [], []), _)]),
	std::vector<unsigned char>* std_vectorLunsigned_charG_clone_const(const std::vector<unsigned char>* instance) {
			std::vector<unsigned char> ret = std::vector<unsigned char>(*instance);
			return new std::vector<unsigned char>(ret);
	}

	// std::vector<unsigned char>::data() generated
	// ("std::vector<unsigned char>::data", vec![(pred!(const, [], []), _)]),
	const unsigned char* std_vectorLunsigned_charG_data_const(const std::vector<unsigned char>* instance) {
			const unsigned char* ret = instance->data();
			return ret;
	}

	// std::vector<unsigned char>::dataMut() generated
	// ("std::vector<unsigned char>::dataMut", vec![(pred!(mut, [], []), _)]),
	unsigned char* std_vectorLunsigned_charG_dataMut(std::vector<unsigned char>* instance) {
			unsigned char* ret = instance->data();
			return ret;
	}

	// cv::fromSlice(Indirect, Primitive) generated
	// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const unsigned char*", "size_t"]), _)]),
	std::vector<unsigned char>* cv_fromSlice_const_const_unsigned_charX_size_t(const unsigned char* data, size_t len) {
			return new std::vector<unsigned char>(data, data + len);
	}

	// std::vector<unsigned char>::inputArray() generated
	// ("std::vector<unsigned char>::inputArray", vec![(pred!(const, [], []), _)]),
	void std_vectorLunsigned_charG_inputArray_const(const std::vector<unsigned char>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			const cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new const cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<unsigned char>::outputArray() generated
	// ("std::vector<unsigned char>::outputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLunsigned_charG_outputArray(std::vector<unsigned char>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<unsigned char>::inputOutputArray() generated
	// ("std::vector<unsigned char>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLunsigned_charG_inputOutputArray(std::vector<unsigned char>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

}


