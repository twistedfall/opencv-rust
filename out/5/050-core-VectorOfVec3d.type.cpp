extern "C" {
	// std::vector<cv::Vec3d>::new() generated
	// ("std::vector<cv::Vec3d>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Vec3d>* std_vectorLcv_Vec3dG_new_const() {
			std::vector<cv::Vec3d>* ret = new std::vector<cv::Vec3d>();
			return ret;
	}

	// std::vector<cv::Vec3d>::delete() generated
	// ("std::vector<cv::Vec3d>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_Vec3dG_delete(std::vector<cv::Vec3d>* instance) {
			delete instance;
	}

	// std::vector<cv::Vec3d>::len() generated
	// ("std::vector<cv::Vec3d>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_Vec3dG_len_const(const std::vector<cv::Vec3d>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::Vec3d>::isEmpty() generated
	// ("std::vector<cv::Vec3d>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_Vec3dG_isEmpty_const(const std::vector<cv::Vec3d>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::Vec3d>::capacity() generated
	// ("std::vector<cv::Vec3d>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_Vec3dG_capacity_const(const std::vector<cv::Vec3d>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::Vec3d>::shrinkToFit() generated
	// ("std::vector<cv::Vec3d>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_Vec3dG_shrinkToFit(std::vector<cv::Vec3d>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::Vec3d>::reserve(Primitive) generated
	// ("std::vector<cv::Vec3d>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_Vec3dG_reserve_size_t(std::vector<cv::Vec3d>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::Vec3d>::remove(Primitive) generated
	// ("std::vector<cv::Vec3d>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_Vec3dG_remove_size_t(std::vector<cv::Vec3d>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::Vec3d>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::Vec3d>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_Vec3dG_swap_size_t_size_t(std::vector<cv::Vec3d>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::Vec3d>::clear() generated
	// ("std::vector<cv::Vec3d>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_Vec3dG_clear(std::vector<cv::Vec3d>* instance) {
			instance->clear();
	}

	// std::vector<cv::Vec3d>::push(SimpleClass) generated
	// ("std::vector<cv::Vec3d>::push", vec![(pred!(mut, ["val"], ["const cv::Vec3d"]), _)]),
	void std_vectorLcv_Vec3dG_push_const_Vec3d(std::vector<cv::Vec3d>* instance, const cv::Vec3d* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::Vec3d>::insert(Primitive, SimpleClass) generated
	// ("std::vector<cv::Vec3d>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Vec3d"]), _)]),
	void std_vectorLcv_Vec3dG_insert_size_t_const_Vec3d(std::vector<cv::Vec3d>* instance, size_t index, const cv::Vec3d* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::Vec3d>::get(Primitive) generated
	// ("std::vector<cv::Vec3d>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_Vec3dG_get_const_size_t(const std::vector<cv::Vec3d>* instance, size_t index, cv::Vec3d* ocvrs_return) {
			cv::Vec3d ret = (*instance)[index];
			*ocvrs_return = ret;
	}

	// std::vector<cv::Vec3d>::set(Primitive, SimpleClass) generated
	// ("std::vector<cv::Vec3d>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Vec3d"]), _)]),
	void std_vectorLcv_Vec3dG_set_size_t_const_Vec3d(std::vector<cv::Vec3d>* instance, size_t index, const cv::Vec3d* val) {
			(*instance)[index] = *val;
	}

	// std::vector<cv::Vec3d>::clone() generated
	// ("std::vector<cv::Vec3d>::clone", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Vec3d>* std_vectorLcv_Vec3dG_clone_const(const std::vector<cv::Vec3d>* instance) {
			std::vector<cv::Vec3d> ret = std::vector<cv::Vec3d>(*instance);
			return new std::vector<cv::Vec3d>(ret);
	}

	// std::vector<cv::Vec3d>::data() generated
	// ("std::vector<cv::Vec3d>::data", vec![(pred!(const, [], []), _)]),
	const cv::Vec3d* std_vectorLcv_Vec3dG_data_const(const std::vector<cv::Vec3d>* instance) {
			const cv::Vec3d* ret = instance->data();
			return ret;
	}

	// std::vector<cv::Vec3d>::dataMut() generated
	// ("std::vector<cv::Vec3d>::dataMut", vec![(pred!(mut, [], []), _)]),
	cv::Vec3d* std_vectorLcv_Vec3dG_dataMut(std::vector<cv::Vec3d>* instance) {
			cv::Vec3d* ret = instance->data();
			return ret;
	}

	// cv::fromSlice(SimpleClass, Primitive) generated
	// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const cv::Vec3d*", "size_t"]), _)]),
	std::vector<cv::Vec3d>* cv_fromSlice_const_const_Vec3dX_size_t(const cv::Vec3d* data, size_t len) {
			return new std::vector<cv::Vec3d>(data, data + len);
	}

	// std::vector<cv::Vec3d>::inputArray() generated
	// ("std::vector<cv::Vec3d>::inputArray", vec![(pred!(const, [], []), _)]),
	void std_vectorLcv_Vec3dG_inputArray_const(const std::vector<cv::Vec3d>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			const cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new const cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<cv::Vec3d>::outputArray() generated
	// ("std::vector<cv::Vec3d>::outputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_Vec3dG_outputArray(std::vector<cv::Vec3d>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<cv::Vec3d>::inputOutputArray() generated
	// ("std::vector<cv::Vec3d>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_Vec3dG_inputOutputArray(std::vector<cv::Vec3d>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

}


