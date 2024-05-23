extern "C" {
	// std::vector<cv::Point>::new() generated
	// ("std::vector<cv::Point>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Point>* std_vectorLcv_PointG_new_const() {
			std::vector<cv::Point>* ret = new std::vector<cv::Point>();
			return ret;
	}

	// std::vector<cv::Point>::delete() generated
	// ("std::vector<cv::Point>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_PointG_delete(std::vector<cv::Point>* instance) {
			delete instance;
	}

	// std::vector<cv::Point>::len() generated
	// ("std::vector<cv::Point>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_PointG_len_const(const std::vector<cv::Point>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::Point>::isEmpty() generated
	// ("std::vector<cv::Point>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_PointG_isEmpty_const(const std::vector<cv::Point>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::Point>::capacity() generated
	// ("std::vector<cv::Point>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_PointG_capacity_const(const std::vector<cv::Point>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::Point>::shrinkToFit() generated
	// ("std::vector<cv::Point>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_PointG_shrinkToFit(std::vector<cv::Point>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::Point>::reserve(Primitive) generated
	// ("std::vector<cv::Point>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_PointG_reserve_size_t(std::vector<cv::Point>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::Point>::remove(Primitive) generated
	// ("std::vector<cv::Point>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_PointG_remove_size_t(std::vector<cv::Point>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::Point>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::Point>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_PointG_swap_size_t_size_t(std::vector<cv::Point>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::Point>::clear() generated
	// ("std::vector<cv::Point>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_PointG_clear(std::vector<cv::Point>* instance) {
			instance->clear();
	}

	// std::vector<cv::Point>::push(SimpleClass) generated
	// ("std::vector<cv::Point>::push", vec![(pred!(mut, ["val"], ["const cv::Point"]), _)]),
	void std_vectorLcv_PointG_push_const_Point(std::vector<cv::Point>* instance, const cv::Point* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::Point>::insert(Primitive, SimpleClass) generated
	// ("std::vector<cv::Point>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Point"]), _)]),
	void std_vectorLcv_PointG_insert_size_t_const_Point(std::vector<cv::Point>* instance, size_t index, const cv::Point* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::Point>::get(Primitive) generated
	// ("std::vector<cv::Point>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_PointG_get_const_size_t(const std::vector<cv::Point>* instance, size_t index, cv::Point* ocvrs_return) {
			cv::Point ret = (*instance)[index];
			*ocvrs_return = ret;
	}

	// std::vector<cv::Point>::set(Primitive, SimpleClass) generated
	// ("std::vector<cv::Point>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Point"]), _)]),
	void std_vectorLcv_PointG_set_size_t_const_Point(std::vector<cv::Point>* instance, size_t index, const cv::Point* val) {
			(*instance)[index] = *val;
	}

	// std::vector<cv::Point>::clone() generated
	// ("std::vector<cv::Point>::clone", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Point>* std_vectorLcv_PointG_clone_const(const std::vector<cv::Point>* instance) {
			std::vector<cv::Point> ret = std::vector<cv::Point>(*instance);
			return new std::vector<cv::Point>(ret);
	}

	// std::vector<cv::Point>::data() generated
	// ("std::vector<cv::Point>::data", vec![(pred!(const, [], []), _)]),
	const cv::Point* std_vectorLcv_PointG_data_const(const std::vector<cv::Point>* instance) {
			const cv::Point* ret = instance->data();
			return ret;
	}

	// std::vector<cv::Point>::dataMut() generated
	// ("std::vector<cv::Point>::dataMut", vec![(pred!(mut, [], []), _)]),
	cv::Point* std_vectorLcv_PointG_dataMut(std::vector<cv::Point>* instance) {
			cv::Point* ret = instance->data();
			return ret;
	}

	// cv::fromSlice(SimpleClass, Primitive) generated
	// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const cv::Point*", "size_t"]), _)]),
	std::vector<cv::Point>* cv_fromSlice_const_const_PointX_size_t(const cv::Point* data, size_t len) {
			return new std::vector<cv::Point>(data, data + len);
	}

	// std::vector<cv::Point>::inputArray() generated
	// ("std::vector<cv::Point>::inputArray", vec![(pred!(const, [], []), _)]),
	void std_vectorLcv_PointG_inputArray_const(const std::vector<cv::Point>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			const cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new const cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<cv::Point>::outputArray() generated
	// ("std::vector<cv::Point>::outputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_PointG_outputArray(std::vector<cv::Point>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<cv::Point>::inputOutputArray() generated
	// ("std::vector<cv::Point>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_PointG_inputOutputArray(std::vector<cv::Point>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

}


