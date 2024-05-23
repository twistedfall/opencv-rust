extern "C" {
	// std::vector<std::vector<cv::Rect>>::new() generated
	// ("std::vector<std::vector<cv::Rect>>::new", vec![(pred!(const, [], []), _)]),
	std::vector<std::vector<cv::Rect>>* std_vectorLstd_vectorLcv_RectGG_new_const() {
			std::vector<std::vector<cv::Rect>>* ret = new std::vector<std::vector<cv::Rect>>();
			return ret;
	}

	// std::vector<std::vector<cv::Rect>>::delete() generated
	// ("std::vector<std::vector<cv::Rect>>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_RectGG_delete(std::vector<std::vector<cv::Rect>>* instance) {
			delete instance;
	}

	// std::vector<std::vector<cv::Rect>>::len() generated
	// ("std::vector<std::vector<cv::Rect>>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_vectorLcv_RectGG_len_const(const std::vector<std::vector<cv::Rect>>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<std::vector<cv::Rect>>::isEmpty() generated
	// ("std::vector<std::vector<cv::Rect>>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLstd_vectorLcv_RectGG_isEmpty_const(const std::vector<std::vector<cv::Rect>>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<std::vector<cv::Rect>>::capacity() generated
	// ("std::vector<std::vector<cv::Rect>>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_vectorLcv_RectGG_capacity_const(const std::vector<std::vector<cv::Rect>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<std::vector<cv::Rect>>::shrinkToFit() generated
	// ("std::vector<std::vector<cv::Rect>>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_RectGG_shrinkToFit(std::vector<std::vector<cv::Rect>>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<std::vector<cv::Rect>>::reserve(Primitive) generated
	// ("std::vector<std::vector<cv::Rect>>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLcv_RectGG_reserve_size_t(std::vector<std::vector<cv::Rect>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<std::vector<cv::Rect>>::remove(Primitive) generated
	// ("std::vector<std::vector<cv::Rect>>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLcv_RectGG_remove_size_t(std::vector<std::vector<cv::Rect>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<std::vector<cv::Rect>>::swap(Primitive, Primitive) generated
	// ("std::vector<std::vector<cv::Rect>>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLstd_vectorLcv_RectGG_swap_size_t_size_t(std::vector<std::vector<cv::Rect>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<std::vector<cv::Rect>>::clear() generated
	// ("std::vector<std::vector<cv::Rect>>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_RectGG_clear(std::vector<std::vector<cv::Rect>>* instance) {
			instance->clear();
	}

	// std::vector<std::vector<cv::Rect>>::push(CppPassByVoidPtr) generated
	// ("std::vector<std::vector<cv::Rect>>::push", vec![(pred!(mut, ["val"], ["const std::vector<cv::Rect>"]), _)]),
	void std_vectorLstd_vectorLcv_RectGG_push_const_vectorLRectG(std::vector<std::vector<cv::Rect>>* instance, const std::vector<cv::Rect>* val) {
			instance->push_back(*val);
	}

	// std::vector<std::vector<cv::Rect>>::insert(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::vector<cv::Rect>>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::vector<cv::Rect>"]), _)]),
	void std_vectorLstd_vectorLcv_RectGG_insert_size_t_const_vectorLRectG(std::vector<std::vector<cv::Rect>>* instance, size_t index, const std::vector<cv::Rect>* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<std::vector<cv::Rect>>::get(Primitive) generated
	// ("std::vector<std::vector<cv::Rect>>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLcv_RectGG_get_const_size_t(const std::vector<std::vector<cv::Rect>>* instance, size_t index, std::vector<cv::Rect>** ocvrs_return) {
			std::vector<cv::Rect> ret = (*instance)[index];
			*ocvrs_return = new std::vector<cv::Rect>(ret);
	}

	// std::vector<std::vector<cv::Rect>>::set(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::vector<cv::Rect>>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::vector<cv::Rect>"]), _)]),
	void std_vectorLstd_vectorLcv_RectGG_set_size_t_const_vectorLRectG(std::vector<std::vector<cv::Rect>>* instance, size_t index, const std::vector<cv::Rect>* val) {
			(*instance)[index] = *val;
	}

	// std::vector<std::vector<cv::Rect>>::inputArray() generated
	// ("std::vector<std::vector<cv::Rect>>::inputArray", vec![(pred!(const, [], []), _)]),
	void std_vectorLstd_vectorLcv_RectGG_inputArray_const(const std::vector<std::vector<cv::Rect>>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			const cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new const cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<std::vector<cv::Rect>>::outputArray() generated
	// ("std::vector<std::vector<cv::Rect>>::outputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_RectGG_outputArray(std::vector<std::vector<cv::Rect>>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<std::vector<cv::Rect>>::inputOutputArray() generated
	// ("std::vector<std::vector<cv::Rect>>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_RectGG_inputOutputArray(std::vector<std::vector<cv::Rect>>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

}


