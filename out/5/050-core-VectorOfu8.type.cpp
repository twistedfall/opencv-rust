extern "C" {
	// std::vector<uint8_t>::new() generated
	// ("std::vector<uint8_t>::new", vec![(pred!(const, [], []), _)]),
	std::vector<uint8_t>* std_vectorLuint8_tG_new_const() {
			std::vector<uint8_t>* ret = new std::vector<uint8_t>();
			return ret;
	}

	// std::vector<uint8_t>::delete() generated
	// ("std::vector<uint8_t>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLuint8_tG_delete(std::vector<uint8_t>* instance) {
			delete instance;
	}

	// std::vector<uint8_t>::len() generated
	// ("std::vector<uint8_t>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLuint8_tG_len_const(const std::vector<uint8_t>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<uint8_t>::isEmpty() generated
	// ("std::vector<uint8_t>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLuint8_tG_isEmpty_const(const std::vector<uint8_t>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<uint8_t>::capacity() generated
	// ("std::vector<uint8_t>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLuint8_tG_capacity_const(const std::vector<uint8_t>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<uint8_t>::shrinkToFit() generated
	// ("std::vector<uint8_t>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLuint8_tG_shrinkToFit(std::vector<uint8_t>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<uint8_t>::reserve(Primitive) generated
	// ("std::vector<uint8_t>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLuint8_tG_reserve_size_t(std::vector<uint8_t>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<uint8_t>::remove(Primitive) generated
	// ("std::vector<uint8_t>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLuint8_tG_remove_size_t(std::vector<uint8_t>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<uint8_t>::swap(Primitive, Primitive) generated
	// ("std::vector<uint8_t>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLuint8_tG_swap_size_t_size_t(std::vector<uint8_t>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<uint8_t>::clear() generated
	// ("std::vector<uint8_t>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLuint8_tG_clear(std::vector<uint8_t>* instance) {
			instance->clear();
	}

	// std::vector<uint8_t>::push(Primitive) generated
	// ("std::vector<uint8_t>::push", vec![(pred!(mut, ["val"], ["const uint8_t"]), _)]),
	void std_vectorLuint8_tG_push_const_uint8_t(std::vector<uint8_t>* instance, const uint8_t val) {
			instance->push_back(val);
	}

	// std::vector<uint8_t>::insert(Primitive, Primitive) generated
	// ("std::vector<uint8_t>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const uint8_t"]), _)]),
	void std_vectorLuint8_tG_insert_size_t_const_uint8_t(std::vector<uint8_t>* instance, size_t index, const uint8_t val) {
			instance->insert(instance->begin() + index, val);
	}

	// std::vector<uint8_t>::get(Primitive) generated
	// ("std::vector<uint8_t>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLuint8_tG_get_const_size_t(const std::vector<uint8_t>* instance, size_t index, uint8_t* ocvrs_return) {
			uint8_t ret = (*instance)[index];
			*ocvrs_return = ret;
	}

	// std::vector<uint8_t>::set(Primitive, Primitive) generated
	// ("std::vector<uint8_t>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const uint8_t"]), _)]),
	void std_vectorLuint8_tG_set_size_t_const_uint8_t(std::vector<uint8_t>* instance, size_t index, const uint8_t val) {
			(*instance)[index] = val;
	}

	// std::vector<uint8_t>::clone() generated
	// ("std::vector<uint8_t>::clone", vec![(pred!(const, [], []), _)]),
	std::vector<uint8_t>* std_vectorLuint8_tG_clone_const(const std::vector<uint8_t>* instance) {
			std::vector<uint8_t> ret = std::vector<uint8_t>(*instance);
			return new std::vector<uint8_t>(ret);
	}

	// std::vector<uint8_t>::data() generated
	// ("std::vector<uint8_t>::data", vec![(pred!(const, [], []), _)]),
	const uint8_t* std_vectorLuint8_tG_data_const(const std::vector<uint8_t>* instance) {
			const uint8_t* ret = instance->data();
			return ret;
	}

	// std::vector<uint8_t>::dataMut() generated
	// ("std::vector<uint8_t>::dataMut", vec![(pred!(mut, [], []), _)]),
	uint8_t* std_vectorLuint8_tG_dataMut(std::vector<uint8_t>* instance) {
			uint8_t* ret = instance->data();
			return ret;
	}

	// cv::fromSlice(Indirect, Primitive) generated
	// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const uint8_t*", "size_t"]), _)]),
	std::vector<uint8_t>* cv_fromSlice_const_const_uint8_tX_size_t(const uint8_t* data, size_t len) {
			return new std::vector<uint8_t>(data, data + len);
	}

	// std::vector<uint8_t>::inputArray() generated
	// ("std::vector<uint8_t>::inputArray", vec![(pred!(const, [], []), _)]),
	void std_vectorLuint8_tG_inputArray_const(const std::vector<uint8_t>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			const cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new const cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<uint8_t>::outputArray() generated
	// ("std::vector<uint8_t>::outputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLuint8_tG_outputArray(std::vector<uint8_t>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<uint8_t>::inputOutputArray() generated
	// ("std::vector<uint8_t>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLuint8_tG_inputOutputArray(std::vector<uint8_t>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

}


