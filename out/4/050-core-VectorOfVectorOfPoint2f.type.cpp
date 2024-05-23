extern "C" {
	// std::vector<std::vector<cv::Point2f>>::new() generated
	// ("std::vector<std::vector<cv::Point2f>>::new", vec![(pred!(const, [], []), _)]),
	std::vector<std::vector<cv::Point2f>>* std_vectorLstd_vectorLcv_Point2fGG_new_const() {
			std::vector<std::vector<cv::Point2f>>* ret = new std::vector<std::vector<cv::Point2f>>();
			return ret;
	}

	// std::vector<std::vector<cv::Point2f>>::delete() generated
	// ("std::vector<std::vector<cv::Point2f>>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_Point2fGG_delete(std::vector<std::vector<cv::Point2f>>* instance) {
			delete instance;
	}

	// std::vector<std::vector<cv::Point2f>>::len() generated
	// ("std::vector<std::vector<cv::Point2f>>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_vectorLcv_Point2fGG_len_const(const std::vector<std::vector<cv::Point2f>>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<std::vector<cv::Point2f>>::isEmpty() generated
	// ("std::vector<std::vector<cv::Point2f>>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLstd_vectorLcv_Point2fGG_isEmpty_const(const std::vector<std::vector<cv::Point2f>>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<std::vector<cv::Point2f>>::capacity() generated
	// ("std::vector<std::vector<cv::Point2f>>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_vectorLcv_Point2fGG_capacity_const(const std::vector<std::vector<cv::Point2f>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<std::vector<cv::Point2f>>::shrinkToFit() generated
	// ("std::vector<std::vector<cv::Point2f>>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_Point2fGG_shrinkToFit(std::vector<std::vector<cv::Point2f>>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<std::vector<cv::Point2f>>::reserve(Primitive) generated
	// ("std::vector<std::vector<cv::Point2f>>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLcv_Point2fGG_reserve_size_t(std::vector<std::vector<cv::Point2f>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<std::vector<cv::Point2f>>::remove(Primitive) generated
	// ("std::vector<std::vector<cv::Point2f>>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLcv_Point2fGG_remove_size_t(std::vector<std::vector<cv::Point2f>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<std::vector<cv::Point2f>>::swap(Primitive, Primitive) generated
	// ("std::vector<std::vector<cv::Point2f>>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLstd_vectorLcv_Point2fGG_swap_size_t_size_t(std::vector<std::vector<cv::Point2f>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<std::vector<cv::Point2f>>::clear() generated
	// ("std::vector<std::vector<cv::Point2f>>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_Point2fGG_clear(std::vector<std::vector<cv::Point2f>>* instance) {
			instance->clear();
	}

	// std::vector<std::vector<cv::Point2f>>::push(CppPassByVoidPtr) generated
	// ("std::vector<std::vector<cv::Point2f>>::push", vec![(pred!(mut, ["val"], ["const std::vector<cv::Point2f>"]), _)]),
	void std_vectorLstd_vectorLcv_Point2fGG_push_const_vectorLPoint2fG(std::vector<std::vector<cv::Point2f>>* instance, const std::vector<cv::Point2f>* val) {
			instance->push_back(*val);
	}

	// std::vector<std::vector<cv::Point2f>>::insert(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::vector<cv::Point2f>>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::vector<cv::Point2f>"]), _)]),
	void std_vectorLstd_vectorLcv_Point2fGG_insert_size_t_const_vectorLPoint2fG(std::vector<std::vector<cv::Point2f>>* instance, size_t index, const std::vector<cv::Point2f>* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<std::vector<cv::Point2f>>::get(Primitive) generated
	// ("std::vector<std::vector<cv::Point2f>>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLcv_Point2fGG_get_const_size_t(const std::vector<std::vector<cv::Point2f>>* instance, size_t index, std::vector<cv::Point2f>** ocvrs_return) {
			std::vector<cv::Point2f> ret = (*instance)[index];
			*ocvrs_return = new std::vector<cv::Point2f>(ret);
	}

	// std::vector<std::vector<cv::Point2f>>::set(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::vector<cv::Point2f>>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::vector<cv::Point2f>"]), _)]),
	void std_vectorLstd_vectorLcv_Point2fGG_set_size_t_const_vectorLPoint2fG(std::vector<std::vector<cv::Point2f>>* instance, size_t index, const std::vector<cv::Point2f>* val) {
			(*instance)[index] = *val;
	}

	// std::vector<std::vector<cv::Point2f>>::inputArray() generated
	// ("std::vector<std::vector<cv::Point2f>>::inputArray", vec![(pred!(const, [], []), _)]),
	void std_vectorLstd_vectorLcv_Point2fGG_inputArray_const(const std::vector<std::vector<cv::Point2f>>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			const cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new const cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<std::vector<cv::Point2f>>::outputArray() generated
	// ("std::vector<std::vector<cv::Point2f>>::outputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_Point2fGG_outputArray(std::vector<std::vector<cv::Point2f>>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<std::vector<cv::Point2f>>::inputOutputArray() generated
	// ("std::vector<std::vector<cv::Point2f>>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_Point2fGG_inputOutputArray(std::vector<std::vector<cv::Point2f>>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

}


