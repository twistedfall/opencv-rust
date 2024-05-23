extern "C" {
	// std::vector<cv::Vec3f>::new() generated
	// ("std::vector<cv::Vec3f>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Vec3f>* std_vectorLcv_Vec3fG_new_const() {
			std::vector<cv::Vec3f>* ret = new std::vector<cv::Vec3f>();
			return ret;
	}

	// std::vector<cv::Vec3f>::delete() generated
	// ("std::vector<cv::Vec3f>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_Vec3fG_delete(std::vector<cv::Vec3f>* instance) {
			delete instance;
	}

	// std::vector<cv::Vec3f>::len() generated
	// ("std::vector<cv::Vec3f>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_Vec3fG_len_const(const std::vector<cv::Vec3f>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::Vec3f>::isEmpty() generated
	// ("std::vector<cv::Vec3f>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_Vec3fG_isEmpty_const(const std::vector<cv::Vec3f>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::Vec3f>::capacity() generated
	// ("std::vector<cv::Vec3f>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_Vec3fG_capacity_const(const std::vector<cv::Vec3f>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::Vec3f>::shrinkToFit() generated
	// ("std::vector<cv::Vec3f>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_Vec3fG_shrinkToFit(std::vector<cv::Vec3f>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::Vec3f>::reserve(Primitive) generated
	// ("std::vector<cv::Vec3f>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_Vec3fG_reserve_size_t(std::vector<cv::Vec3f>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::Vec3f>::remove(Primitive) generated
	// ("std::vector<cv::Vec3f>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_Vec3fG_remove_size_t(std::vector<cv::Vec3f>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::Vec3f>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::Vec3f>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_Vec3fG_swap_size_t_size_t(std::vector<cv::Vec3f>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::Vec3f>::clear() generated
	// ("std::vector<cv::Vec3f>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_Vec3fG_clear(std::vector<cv::Vec3f>* instance) {
			instance->clear();
	}

	// std::vector<cv::Vec3f>::push(SimpleClass) generated
	// ("std::vector<cv::Vec3f>::push", vec![(pred!(mut, ["val"], ["const cv::Vec3f"]), _)]),
	void std_vectorLcv_Vec3fG_push_const_Vec3f(std::vector<cv::Vec3f>* instance, const cv::Vec3f* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::Vec3f>::insert(Primitive, SimpleClass) generated
	// ("std::vector<cv::Vec3f>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Vec3f"]), _)]),
	void std_vectorLcv_Vec3fG_insert_size_t_const_Vec3f(std::vector<cv::Vec3f>* instance, size_t index, const cv::Vec3f* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::Vec3f>::get(Primitive) generated
	// ("std::vector<cv::Vec3f>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_Vec3fG_get_const_size_t(const std::vector<cv::Vec3f>* instance, size_t index, cv::Vec3f* ocvrs_return) {
			cv::Vec3f ret = (*instance)[index];
			*ocvrs_return = ret;
	}

	// std::vector<cv::Vec3f>::set(Primitive, SimpleClass) generated
	// ("std::vector<cv::Vec3f>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Vec3f"]), _)]),
	void std_vectorLcv_Vec3fG_set_size_t_const_Vec3f(std::vector<cv::Vec3f>* instance, size_t index, const cv::Vec3f* val) {
			(*instance)[index] = *val;
	}

	// std::vector<cv::Vec3f>::clone() generated
	// ("std::vector<cv::Vec3f>::clone", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Vec3f>* std_vectorLcv_Vec3fG_clone_const(const std::vector<cv::Vec3f>* instance) {
			std::vector<cv::Vec3f> ret = std::vector<cv::Vec3f>(*instance);
			return new std::vector<cv::Vec3f>(ret);
	}

	// std::vector<cv::Vec3f>::data() generated
	// ("std::vector<cv::Vec3f>::data", vec![(pred!(const, [], []), _)]),
	const cv::Vec3f* std_vectorLcv_Vec3fG_data_const(const std::vector<cv::Vec3f>* instance) {
			const cv::Vec3f* ret = instance->data();
			return ret;
	}

	// std::vector<cv::Vec3f>::dataMut() generated
	// ("std::vector<cv::Vec3f>::dataMut", vec![(pred!(mut, [], []), _)]),
	cv::Vec3f* std_vectorLcv_Vec3fG_dataMut(std::vector<cv::Vec3f>* instance) {
			cv::Vec3f* ret = instance->data();
			return ret;
	}

	// cv::fromSlice(SimpleClass, Primitive) generated
	// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const cv::Vec3f*", "size_t"]), _)]),
	std::vector<cv::Vec3f>* cv_fromSlice_const_const_Vec3fX_size_t(const cv::Vec3f* data, size_t len) {
			return new std::vector<cv::Vec3f>(data, data + len);
	}

	// std::vector<cv::Vec3f>::inputArray() generated
	// ("std::vector<cv::Vec3f>::inputArray", vec![(pred!(const, [], []), _)]),
	void std_vectorLcv_Vec3fG_inputArray_const(const std::vector<cv::Vec3f>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			const cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new const cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<cv::Vec3f>::outputArray() generated
	// ("std::vector<cv::Vec3f>::outputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_Vec3fG_outputArray(std::vector<cv::Vec3f>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<cv::Vec3f>::inputOutputArray() generated
	// ("std::vector<cv::Vec3f>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_Vec3fG_inputOutputArray(std::vector<cv::Vec3f>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

}


