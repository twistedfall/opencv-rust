extern "C" {
	// std::vector<cv::Point2d>::new() generated
	// ("std::vector<cv::Point2d>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Point2d>* std_vectorLcv_Point2dG_new_const() {
			std::vector<cv::Point2d>* ret = new std::vector<cv::Point2d>();
			return ret;
	}

	// std::vector<cv::Point2d>::delete() generated
	// ("std::vector<cv::Point2d>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_Point2dG_delete(std::vector<cv::Point2d>* instance) {
			delete instance;
	}

	// std::vector<cv::Point2d>::len() generated
	// ("std::vector<cv::Point2d>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_Point2dG_len_const(const std::vector<cv::Point2d>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::Point2d>::isEmpty() generated
	// ("std::vector<cv::Point2d>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_Point2dG_isEmpty_const(const std::vector<cv::Point2d>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::Point2d>::capacity() generated
	// ("std::vector<cv::Point2d>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_Point2dG_capacity_const(const std::vector<cv::Point2d>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::Point2d>::shrinkToFit() generated
	// ("std::vector<cv::Point2d>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_Point2dG_shrinkToFit(std::vector<cv::Point2d>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::Point2d>::reserve(Primitive) generated
	// ("std::vector<cv::Point2d>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_Point2dG_reserve_size_t(std::vector<cv::Point2d>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::Point2d>::remove(Primitive) generated
	// ("std::vector<cv::Point2d>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_Point2dG_remove_size_t(std::vector<cv::Point2d>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::Point2d>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::Point2d>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_Point2dG_swap_size_t_size_t(std::vector<cv::Point2d>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::Point2d>::clear() generated
	// ("std::vector<cv::Point2d>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_Point2dG_clear(std::vector<cv::Point2d>* instance) {
			instance->clear();
	}

	// std::vector<cv::Point2d>::push(SimpleClass) generated
	// ("std::vector<cv::Point2d>::push", vec![(pred!(mut, ["val"], ["const cv::Point2d"]), _)]),
	void std_vectorLcv_Point2dG_push_const_Point2d(std::vector<cv::Point2d>* instance, const cv::Point2d* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::Point2d>::insert(Primitive, SimpleClass) generated
	// ("std::vector<cv::Point2d>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Point2d"]), _)]),
	void std_vectorLcv_Point2dG_insert_size_t_const_Point2d(std::vector<cv::Point2d>* instance, size_t index, const cv::Point2d* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::Point2d>::get(Primitive) generated
	// ("std::vector<cv::Point2d>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_Point2dG_get_const_size_t(const std::vector<cv::Point2d>* instance, size_t index, cv::Point2d* ocvrs_return) {
			cv::Point2d ret = (*instance)[index];
			*ocvrs_return = ret;
	}

	// std::vector<cv::Point2d>::set(Primitive, SimpleClass) generated
	// ("std::vector<cv::Point2d>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Point2d"]), _)]),
	void std_vectorLcv_Point2dG_set_size_t_const_Point2d(std::vector<cv::Point2d>* instance, size_t index, const cv::Point2d* val) {
			(*instance)[index] = *val;
	}

	// std::vector<cv::Point2d>::clone() generated
	// ("std::vector<cv::Point2d>::clone", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Point2d>* std_vectorLcv_Point2dG_clone_const(const std::vector<cv::Point2d>* instance) {
			std::vector<cv::Point2d> ret = std::vector<cv::Point2d>(*instance);
			return new std::vector<cv::Point2d>(ret);
	}

	// std::vector<cv::Point2d>::data() generated
	// ("std::vector<cv::Point2d>::data", vec![(pred!(const, [], []), _)]),
	const cv::Point2d* std_vectorLcv_Point2dG_data_const(const std::vector<cv::Point2d>* instance) {
			const cv::Point2d* ret = instance->data();
			return ret;
	}

	// std::vector<cv::Point2d>::dataMut() generated
	// ("std::vector<cv::Point2d>::dataMut", vec![(pred!(mut, [], []), _)]),
	cv::Point2d* std_vectorLcv_Point2dG_dataMut(std::vector<cv::Point2d>* instance) {
			cv::Point2d* ret = instance->data();
			return ret;
	}

	// cv::fromSlice(SimpleClass, Primitive) generated
	// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const cv::Point2d*", "size_t"]), _)]),
	std::vector<cv::Point2d>* cv_fromSlice_const_const_Point2dX_size_t(const cv::Point2d* data, size_t len) {
			return new std::vector<cv::Point2d>(data, data + len);
	}

	// std::vector<cv::Point2d>::inputArray() generated
	// ("std::vector<cv::Point2d>::inputArray", vec![(pred!(const, [], []), _)]),
	void std_vectorLcv_Point2dG_inputArray_const(const std::vector<cv::Point2d>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			const cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new const cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<cv::Point2d>::outputArray() generated
	// ("std::vector<cv::Point2d>::outputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_Point2dG_outputArray(std::vector<cv::Point2d>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<cv::Point2d>::inputOutputArray() generated
	// ("std::vector<cv::Point2d>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_Point2dG_inputOutputArray(std::vector<cv::Point2d>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

}


