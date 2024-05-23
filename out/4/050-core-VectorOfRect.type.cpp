extern "C" {
	// std::vector<cv::Rect>::new() generated
	// ("std::vector<cv::Rect>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Rect>* std_vectorLcv_RectG_new_const() {
			std::vector<cv::Rect>* ret = new std::vector<cv::Rect>();
			return ret;
	}

	// std::vector<cv::Rect>::delete() generated
	// ("std::vector<cv::Rect>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_RectG_delete(std::vector<cv::Rect>* instance) {
			delete instance;
	}

	// std::vector<cv::Rect>::len() generated
	// ("std::vector<cv::Rect>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_RectG_len_const(const std::vector<cv::Rect>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::Rect>::isEmpty() generated
	// ("std::vector<cv::Rect>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_RectG_isEmpty_const(const std::vector<cv::Rect>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::Rect>::capacity() generated
	// ("std::vector<cv::Rect>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_RectG_capacity_const(const std::vector<cv::Rect>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::Rect>::shrinkToFit() generated
	// ("std::vector<cv::Rect>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_RectG_shrinkToFit(std::vector<cv::Rect>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::Rect>::reserve(Primitive) generated
	// ("std::vector<cv::Rect>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_RectG_reserve_size_t(std::vector<cv::Rect>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::Rect>::remove(Primitive) generated
	// ("std::vector<cv::Rect>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_RectG_remove_size_t(std::vector<cv::Rect>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::Rect>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::Rect>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_RectG_swap_size_t_size_t(std::vector<cv::Rect>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::Rect>::clear() generated
	// ("std::vector<cv::Rect>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_RectG_clear(std::vector<cv::Rect>* instance) {
			instance->clear();
	}

	// std::vector<cv::Rect>::push(SimpleClass) generated
	// ("std::vector<cv::Rect>::push", vec![(pred!(mut, ["val"], ["const cv::Rect"]), _)]),
	void std_vectorLcv_RectG_push_const_Rect(std::vector<cv::Rect>* instance, const cv::Rect* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::Rect>::insert(Primitive, SimpleClass) generated
	// ("std::vector<cv::Rect>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Rect"]), _)]),
	void std_vectorLcv_RectG_insert_size_t_const_Rect(std::vector<cv::Rect>* instance, size_t index, const cv::Rect* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::Rect>::get(Primitive) generated
	// ("std::vector<cv::Rect>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_RectG_get_const_size_t(const std::vector<cv::Rect>* instance, size_t index, cv::Rect* ocvrs_return) {
			cv::Rect ret = (*instance)[index];
			*ocvrs_return = ret;
	}

	// std::vector<cv::Rect>::set(Primitive, SimpleClass) generated
	// ("std::vector<cv::Rect>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Rect"]), _)]),
	void std_vectorLcv_RectG_set_size_t_const_Rect(std::vector<cv::Rect>* instance, size_t index, const cv::Rect* val) {
			(*instance)[index] = *val;
	}

	// std::vector<cv::Rect>::clone() generated
	// ("std::vector<cv::Rect>::clone", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Rect>* std_vectorLcv_RectG_clone_const(const std::vector<cv::Rect>* instance) {
			std::vector<cv::Rect> ret = std::vector<cv::Rect>(*instance);
			return new std::vector<cv::Rect>(ret);
	}

	// std::vector<cv::Rect>::data() generated
	// ("std::vector<cv::Rect>::data", vec![(pred!(const, [], []), _)]),
	const cv::Rect* std_vectorLcv_RectG_data_const(const std::vector<cv::Rect>* instance) {
			const cv::Rect* ret = instance->data();
			return ret;
	}

	// std::vector<cv::Rect>::dataMut() generated
	// ("std::vector<cv::Rect>::dataMut", vec![(pred!(mut, [], []), _)]),
	cv::Rect* std_vectorLcv_RectG_dataMut(std::vector<cv::Rect>* instance) {
			cv::Rect* ret = instance->data();
			return ret;
	}

	// cv::fromSlice(SimpleClass, Primitive) generated
	// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const cv::Rect*", "size_t"]), _)]),
	std::vector<cv::Rect>* cv_fromSlice_const_const_RectX_size_t(const cv::Rect* data, size_t len) {
			return new std::vector<cv::Rect>(data, data + len);
	}

	// std::vector<cv::Rect>::inputArray() generated
	// ("std::vector<cv::Rect>::inputArray", vec![(pred!(const, [], []), _)]),
	void std_vectorLcv_RectG_inputArray_const(const std::vector<cv::Rect>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			const cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new const cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<cv::Rect>::outputArray() generated
	// ("std::vector<cv::Rect>::outputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_RectG_outputArray(std::vector<cv::Rect>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<cv::Rect>::inputOutputArray() generated
	// ("std::vector<cv::Rect>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_RectG_inputOutputArray(std::vector<cv::Rect>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

}


