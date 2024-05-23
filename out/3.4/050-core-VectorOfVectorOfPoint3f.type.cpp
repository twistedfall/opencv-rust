extern "C" {
	// std::vector<std::vector<cv::Point3f>>::new() generated
	// ("std::vector<std::vector<cv::Point3f>>::new", vec![(pred!(const, [], []), _)]),
	std::vector<std::vector<cv::Point3f>>* std_vectorLstd_vectorLcv_Point3fGG_new_const() {
			std::vector<std::vector<cv::Point3f>>* ret = new std::vector<std::vector<cv::Point3f>>();
			return ret;
	}

	// std::vector<std::vector<cv::Point3f>>::delete() generated
	// ("std::vector<std::vector<cv::Point3f>>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_Point3fGG_delete(std::vector<std::vector<cv::Point3f>>* instance) {
			delete instance;
	}

	// std::vector<std::vector<cv::Point3f>>::len() generated
	// ("std::vector<std::vector<cv::Point3f>>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_vectorLcv_Point3fGG_len_const(const std::vector<std::vector<cv::Point3f>>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<std::vector<cv::Point3f>>::isEmpty() generated
	// ("std::vector<std::vector<cv::Point3f>>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLstd_vectorLcv_Point3fGG_isEmpty_const(const std::vector<std::vector<cv::Point3f>>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<std::vector<cv::Point3f>>::capacity() generated
	// ("std::vector<std::vector<cv::Point3f>>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_vectorLcv_Point3fGG_capacity_const(const std::vector<std::vector<cv::Point3f>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<std::vector<cv::Point3f>>::shrinkToFit() generated
	// ("std::vector<std::vector<cv::Point3f>>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_Point3fGG_shrinkToFit(std::vector<std::vector<cv::Point3f>>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<std::vector<cv::Point3f>>::reserve(Primitive) generated
	// ("std::vector<std::vector<cv::Point3f>>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLcv_Point3fGG_reserve_size_t(std::vector<std::vector<cv::Point3f>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<std::vector<cv::Point3f>>::remove(Primitive) generated
	// ("std::vector<std::vector<cv::Point3f>>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLcv_Point3fGG_remove_size_t(std::vector<std::vector<cv::Point3f>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<std::vector<cv::Point3f>>::swap(Primitive, Primitive) generated
	// ("std::vector<std::vector<cv::Point3f>>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLstd_vectorLcv_Point3fGG_swap_size_t_size_t(std::vector<std::vector<cv::Point3f>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<std::vector<cv::Point3f>>::clear() generated
	// ("std::vector<std::vector<cv::Point3f>>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_Point3fGG_clear(std::vector<std::vector<cv::Point3f>>* instance) {
			instance->clear();
	}

	// std::vector<std::vector<cv::Point3f>>::push(CppPassByVoidPtr) generated
	// ("std::vector<std::vector<cv::Point3f>>::push", vec![(pred!(mut, ["val"], ["const std::vector<cv::Point3f>"]), _)]),
	void std_vectorLstd_vectorLcv_Point3fGG_push_const_vectorLPoint3fG(std::vector<std::vector<cv::Point3f>>* instance, const std::vector<cv::Point3f>* val) {
			instance->push_back(*val);
	}

	// std::vector<std::vector<cv::Point3f>>::insert(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::vector<cv::Point3f>>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::vector<cv::Point3f>"]), _)]),
	void std_vectorLstd_vectorLcv_Point3fGG_insert_size_t_const_vectorLPoint3fG(std::vector<std::vector<cv::Point3f>>* instance, size_t index, const std::vector<cv::Point3f>* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<std::vector<cv::Point3f>>::get(Primitive) generated
	// ("std::vector<std::vector<cv::Point3f>>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLcv_Point3fGG_get_const_size_t(const std::vector<std::vector<cv::Point3f>>* instance, size_t index, std::vector<cv::Point3f>** ocvrs_return) {
			std::vector<cv::Point3f> ret = (*instance)[index];
			*ocvrs_return = new std::vector<cv::Point3f>(ret);
	}

	// std::vector<std::vector<cv::Point3f>>::set(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::vector<cv::Point3f>>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::vector<cv::Point3f>"]), _)]),
	void std_vectorLstd_vectorLcv_Point3fGG_set_size_t_const_vectorLPoint3fG(std::vector<std::vector<cv::Point3f>>* instance, size_t index, const std::vector<cv::Point3f>* val) {
			(*instance)[index] = *val;
	}

	// std::vector<std::vector<cv::Point3f>>::inputArray() generated
	// ("std::vector<std::vector<cv::Point3f>>::inputArray", vec![(pred!(const, [], []), _)]),
	void std_vectorLstd_vectorLcv_Point3fGG_inputArray_const(const std::vector<std::vector<cv::Point3f>>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			const cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new const cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<std::vector<cv::Point3f>>::outputArray() generated
	// ("std::vector<std::vector<cv::Point3f>>::outputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_Point3fGG_outputArray(std::vector<std::vector<cv::Point3f>>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<std::vector<cv::Point3f>>::inputOutputArray() generated
	// ("std::vector<std::vector<cv::Point3f>>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_Point3fGG_inputOutputArray(std::vector<std::vector<cv::Point3f>>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

}


