extern "C" {
	// std::vector<cv::Scalar>::new() generated
	// ("std::vector<cv::Scalar>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Scalar>* std_vectorLcv_ScalarG_new_const() {
			std::vector<cv::Scalar>* ret = new std::vector<cv::Scalar>();
			return ret;
	}

	// std::vector<cv::Scalar>::delete() generated
	// ("std::vector<cv::Scalar>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_ScalarG_delete(std::vector<cv::Scalar>* instance) {
			delete instance;
	}

	// std::vector<cv::Scalar>::len() generated
	// ("std::vector<cv::Scalar>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_ScalarG_len_const(const std::vector<cv::Scalar>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::Scalar>::isEmpty() generated
	// ("std::vector<cv::Scalar>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_ScalarG_isEmpty_const(const std::vector<cv::Scalar>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::Scalar>::capacity() generated
	// ("std::vector<cv::Scalar>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_ScalarG_capacity_const(const std::vector<cv::Scalar>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::Scalar>::shrinkToFit() generated
	// ("std::vector<cv::Scalar>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_ScalarG_shrinkToFit(std::vector<cv::Scalar>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::Scalar>::reserve(Primitive) generated
	// ("std::vector<cv::Scalar>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_ScalarG_reserve_size_t(std::vector<cv::Scalar>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::Scalar>::remove(Primitive) generated
	// ("std::vector<cv::Scalar>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_ScalarG_remove_size_t(std::vector<cv::Scalar>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::Scalar>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::Scalar>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_ScalarG_swap_size_t_size_t(std::vector<cv::Scalar>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::Scalar>::clear() generated
	// ("std::vector<cv::Scalar>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_ScalarG_clear(std::vector<cv::Scalar>* instance) {
			instance->clear();
	}

	// std::vector<cv::Scalar>::push(SimpleClass) generated
	// ("std::vector<cv::Scalar>::push", vec![(pred!(mut, ["val"], ["const cv::Scalar"]), _)]),
	void std_vectorLcv_ScalarG_push_const_Scalar(std::vector<cv::Scalar>* instance, const cv::Scalar* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::Scalar>::insert(Primitive, SimpleClass) generated
	// ("std::vector<cv::Scalar>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Scalar"]), _)]),
	void std_vectorLcv_ScalarG_insert_size_t_const_Scalar(std::vector<cv::Scalar>* instance, size_t index, const cv::Scalar* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::Scalar>::get(Primitive) generated
	// ("std::vector<cv::Scalar>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_ScalarG_get_const_size_t(const std::vector<cv::Scalar>* instance, size_t index, cv::Scalar* ocvrs_return) {
			cv::Scalar ret = (*instance)[index];
			*ocvrs_return = ret;
	}

	// std::vector<cv::Scalar>::set(Primitive, SimpleClass) generated
	// ("std::vector<cv::Scalar>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Scalar"]), _)]),
	void std_vectorLcv_ScalarG_set_size_t_const_Scalar(std::vector<cv::Scalar>* instance, size_t index, const cv::Scalar* val) {
			(*instance)[index] = *val;
	}

	// std::vector<cv::Scalar>::clone() generated
	// ("std::vector<cv::Scalar>::clone", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Scalar>* std_vectorLcv_ScalarG_clone_const(const std::vector<cv::Scalar>* instance) {
			std::vector<cv::Scalar> ret = std::vector<cv::Scalar>(*instance);
			return new std::vector<cv::Scalar>(ret);
	}

	// std::vector<cv::Scalar>::data() generated
	// ("std::vector<cv::Scalar>::data", vec![(pred!(const, [], []), _)]),
	const cv::Scalar* std_vectorLcv_ScalarG_data_const(const std::vector<cv::Scalar>* instance) {
			const cv::Scalar* ret = instance->data();
			return ret;
	}

	// std::vector<cv::Scalar>::dataMut() generated
	// ("std::vector<cv::Scalar>::dataMut", vec![(pred!(mut, [], []), _)]),
	cv::Scalar* std_vectorLcv_ScalarG_dataMut(std::vector<cv::Scalar>* instance) {
			cv::Scalar* ret = instance->data();
			return ret;
	}

	// cv::fromSlice(SimpleClass, Primitive) generated
	// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const cv::Scalar*", "size_t"]), _)]),
	std::vector<cv::Scalar>* cv_fromSlice_const_const_ScalarX_size_t(const cv::Scalar* data, size_t len) {
			return new std::vector<cv::Scalar>(data, data + len);
	}

	// std::vector<cv::Scalar>::inputArray() generated
	// ("std::vector<cv::Scalar>::inputArray", vec![(pred!(const, [], []), _)]),
	void std_vectorLcv_ScalarG_inputArray_const(const std::vector<cv::Scalar>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			const cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new const cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<cv::Scalar>::outputArray() generated
	// ("std::vector<cv::Scalar>::outputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_ScalarG_outputArray(std::vector<cv::Scalar>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<cv::Scalar>::inputOutputArray() generated
	// ("std::vector<cv::Scalar>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_ScalarG_inputOutputArray(std::vector<cv::Scalar>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

}


