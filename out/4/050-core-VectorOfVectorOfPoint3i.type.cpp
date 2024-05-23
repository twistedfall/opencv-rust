extern "C" {
	// std::vector<std::vector<cv::Point3i>>::new() generated
	// ("std::vector<std::vector<cv::Point3i>>::new", vec![(pred!(const, [], []), _)]),
	std::vector<std::vector<cv::Point3i>>* std_vectorLstd_vectorLcv_Point3iGG_new_const() {
			std::vector<std::vector<cv::Point3i>>* ret = new std::vector<std::vector<cv::Point3i>>();
			return ret;
	}

	// std::vector<std::vector<cv::Point3i>>::delete() generated
	// ("std::vector<std::vector<cv::Point3i>>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_Point3iGG_delete(std::vector<std::vector<cv::Point3i>>* instance) {
			delete instance;
	}

	// std::vector<std::vector<cv::Point3i>>::len() generated
	// ("std::vector<std::vector<cv::Point3i>>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_vectorLcv_Point3iGG_len_const(const std::vector<std::vector<cv::Point3i>>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<std::vector<cv::Point3i>>::isEmpty() generated
	// ("std::vector<std::vector<cv::Point3i>>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLstd_vectorLcv_Point3iGG_isEmpty_const(const std::vector<std::vector<cv::Point3i>>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<std::vector<cv::Point3i>>::capacity() generated
	// ("std::vector<std::vector<cv::Point3i>>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_vectorLcv_Point3iGG_capacity_const(const std::vector<std::vector<cv::Point3i>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<std::vector<cv::Point3i>>::shrinkToFit() generated
	// ("std::vector<std::vector<cv::Point3i>>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_Point3iGG_shrinkToFit(std::vector<std::vector<cv::Point3i>>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<std::vector<cv::Point3i>>::reserve(Primitive) generated
	// ("std::vector<std::vector<cv::Point3i>>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLcv_Point3iGG_reserve_size_t(std::vector<std::vector<cv::Point3i>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<std::vector<cv::Point3i>>::remove(Primitive) generated
	// ("std::vector<std::vector<cv::Point3i>>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLcv_Point3iGG_remove_size_t(std::vector<std::vector<cv::Point3i>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<std::vector<cv::Point3i>>::swap(Primitive, Primitive) generated
	// ("std::vector<std::vector<cv::Point3i>>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLstd_vectorLcv_Point3iGG_swap_size_t_size_t(std::vector<std::vector<cv::Point3i>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<std::vector<cv::Point3i>>::clear() generated
	// ("std::vector<std::vector<cv::Point3i>>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_Point3iGG_clear(std::vector<std::vector<cv::Point3i>>* instance) {
			instance->clear();
	}

	// std::vector<std::vector<cv::Point3i>>::push(CppPassByVoidPtr) generated
	// ("std::vector<std::vector<cv::Point3i>>::push", vec![(pred!(mut, ["val"], ["const std::vector<cv::Point3i>"]), _)]),
	void std_vectorLstd_vectorLcv_Point3iGG_push_const_vectorLPoint3iG(std::vector<std::vector<cv::Point3i>>* instance, const std::vector<cv::Point3i>* val) {
			instance->push_back(*val);
	}

	// std::vector<std::vector<cv::Point3i>>::insert(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::vector<cv::Point3i>>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::vector<cv::Point3i>"]), _)]),
	void std_vectorLstd_vectorLcv_Point3iGG_insert_size_t_const_vectorLPoint3iG(std::vector<std::vector<cv::Point3i>>* instance, size_t index, const std::vector<cv::Point3i>* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<std::vector<cv::Point3i>>::get(Primitive) generated
	// ("std::vector<std::vector<cv::Point3i>>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLcv_Point3iGG_get_const_size_t(const std::vector<std::vector<cv::Point3i>>* instance, size_t index, std::vector<cv::Point3i>** ocvrs_return) {
			std::vector<cv::Point3i> ret = (*instance)[index];
			*ocvrs_return = new std::vector<cv::Point3i>(ret);
	}

	// std::vector<std::vector<cv::Point3i>>::set(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::vector<cv::Point3i>>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::vector<cv::Point3i>"]), _)]),
	void std_vectorLstd_vectorLcv_Point3iGG_set_size_t_const_vectorLPoint3iG(std::vector<std::vector<cv::Point3i>>* instance, size_t index, const std::vector<cv::Point3i>* val) {
			(*instance)[index] = *val;
	}

	// std::vector<std::vector<cv::Point3i>>::inputArray() generated
	// ("std::vector<std::vector<cv::Point3i>>::inputArray", vec![(pred!(const, [], []), _)]),
	void std_vectorLstd_vectorLcv_Point3iGG_inputArray_const(const std::vector<std::vector<cv::Point3i>>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			const cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new const cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<std::vector<cv::Point3i>>::outputArray() generated
	// ("std::vector<std::vector<cv::Point3i>>::outputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_Point3iGG_outputArray(std::vector<std::vector<cv::Point3i>>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<std::vector<cv::Point3i>>::inputOutputArray() generated
	// ("std::vector<std::vector<cv::Point3i>>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_Point3iGG_inputOutputArray(std::vector<std::vector<cv::Point3i>>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

}


