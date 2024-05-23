extern "C" {
	// std::vector<cv::dnn::MatType>::new() generated
	// ("std::vector<cv::dnn::MatType>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::dnn::MatType>* std_vectorLcv_dnn_MatTypeG_new_const() {
			std::vector<cv::dnn::MatType>* ret = new std::vector<cv::dnn::MatType>();
			return ret;
	}

	// std::vector<cv::dnn::MatType>::delete() generated
	// ("std::vector<cv::dnn::MatType>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_dnn_MatTypeG_delete(std::vector<cv::dnn::MatType>* instance) {
			delete instance;
	}

	// std::vector<cv::dnn::MatType>::len() generated
	// ("std::vector<cv::dnn::MatType>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_dnn_MatTypeG_len_const(const std::vector<cv::dnn::MatType>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::dnn::MatType>::isEmpty() generated
	// ("std::vector<cv::dnn::MatType>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_dnn_MatTypeG_isEmpty_const(const std::vector<cv::dnn::MatType>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::dnn::MatType>::capacity() generated
	// ("std::vector<cv::dnn::MatType>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_dnn_MatTypeG_capacity_const(const std::vector<cv::dnn::MatType>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::dnn::MatType>::shrinkToFit() generated
	// ("std::vector<cv::dnn::MatType>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_dnn_MatTypeG_shrinkToFit(std::vector<cv::dnn::MatType>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::dnn::MatType>::reserve(Primitive) generated
	// ("std::vector<cv::dnn::MatType>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_dnn_MatTypeG_reserve_size_t(std::vector<cv::dnn::MatType>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::dnn::MatType>::remove(Primitive) generated
	// ("std::vector<cv::dnn::MatType>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_dnn_MatTypeG_remove_size_t(std::vector<cv::dnn::MatType>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::dnn::MatType>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::dnn::MatType>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_dnn_MatTypeG_swap_size_t_size_t(std::vector<cv::dnn::MatType>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::dnn::MatType>::clear() generated
	// ("std::vector<cv::dnn::MatType>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_dnn_MatTypeG_clear(std::vector<cv::dnn::MatType>* instance) {
			instance->clear();
	}

	// std::vector<cv::dnn::MatType>::push(Primitive) generated
	// ("std::vector<cv::dnn::MatType>::push", vec![(pred!(mut, ["val"], ["const cv::dnn::MatType"]), _)]),
	void std_vectorLcv_dnn_MatTypeG_push_const_MatType(std::vector<cv::dnn::MatType>* instance, const cv::dnn::MatType val) {
			instance->push_back(val);
	}

	// std::vector<cv::dnn::MatType>::insert(Primitive, Primitive) generated
	// ("std::vector<cv::dnn::MatType>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::dnn::MatType"]), _)]),
	void std_vectorLcv_dnn_MatTypeG_insert_size_t_const_MatType(std::vector<cv::dnn::MatType>* instance, size_t index, const cv::dnn::MatType val) {
			instance->insert(instance->begin() + index, val);
	}

	// std::vector<cv::dnn::MatType>::get(Primitive) generated
	// ("std::vector<cv::dnn::MatType>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_dnn_MatTypeG_get_const_size_t(const std::vector<cv::dnn::MatType>* instance, size_t index, cv::dnn::MatType* ocvrs_return) {
			cv::dnn::MatType ret = (*instance)[index];
			*ocvrs_return = ret;
	}

	// std::vector<cv::dnn::MatType>::set(Primitive, Primitive) generated
	// ("std::vector<cv::dnn::MatType>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::dnn::MatType"]), _)]),
	void std_vectorLcv_dnn_MatTypeG_set_size_t_const_MatType(std::vector<cv::dnn::MatType>* instance, size_t index, const cv::dnn::MatType val) {
			(*instance)[index] = val;
	}

	// std::vector<cv::dnn::MatType>::clone() generated
	// ("std::vector<cv::dnn::MatType>::clone", vec![(pred!(const, [], []), _)]),
	std::vector<cv::dnn::MatType>* std_vectorLcv_dnn_MatTypeG_clone_const(const std::vector<cv::dnn::MatType>* instance) {
			std::vector<cv::dnn::MatType> ret = std::vector<cv::dnn::MatType>(*instance);
			return new std::vector<cv::dnn::MatType>(ret);
	}

	// std::vector<cv::dnn::MatType>::data() generated
	// ("std::vector<cv::dnn::MatType>::data", vec![(pred!(const, [], []), _)]),
	const cv::dnn::MatType* std_vectorLcv_dnn_MatTypeG_data_const(const std::vector<cv::dnn::MatType>* instance) {
			const cv::dnn::MatType* ret = instance->data();
			return ret;
	}

	// std::vector<cv::dnn::MatType>::dataMut() generated
	// ("std::vector<cv::dnn::MatType>::dataMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::MatType* std_vectorLcv_dnn_MatTypeG_dataMut(std::vector<cv::dnn::MatType>* instance) {
			cv::dnn::MatType* ret = instance->data();
			return ret;
	}

	// cv::fromSlice(Indirect, Primitive) generated
	// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const cv::dnn::MatType*", "size_t"]), _)]),
	std::vector<cv::dnn::MatType>* cv_fromSlice_const_const_MatTypeX_size_t(const cv::dnn::MatType* data, size_t len) {
			return new std::vector<cv::dnn::MatType>(data, data + len);
	}

	// std::vector<cv::dnn::MatType>::inputArray() generated
	// ("std::vector<cv::dnn::MatType>::inputArray", vec![(pred!(const, [], []), _)]),
	void std_vectorLcv_dnn_MatTypeG_inputArray_const(const std::vector<cv::dnn::MatType>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			const cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new const cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<cv::dnn::MatType>::outputArray() generated
	// ("std::vector<cv::dnn::MatType>::outputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_dnn_MatTypeG_outputArray(std::vector<cv::dnn::MatType>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<cv::dnn::MatType>::inputOutputArray() generated
	// ("std::vector<cv::dnn::MatType>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_dnn_MatTypeG_inputOutputArray(std::vector<cv::dnn::MatType>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

}


