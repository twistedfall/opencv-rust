extern "C" {
	// std::vector<std::vector<cv::Vec2d>>::new() generated
	// ("std::vector<std::vector<cv::Vec2d>>::new", vec![(pred!(const, [], []), _)]),
	std::vector<std::vector<cv::Vec2d>>* std_vectorLstd_vectorLcv_Vec2dGG_new_const() {
			std::vector<std::vector<cv::Vec2d>>* ret = new std::vector<std::vector<cv::Vec2d>>();
			return ret;
	}

	// std::vector<std::vector<cv::Vec2d>>::delete() generated
	// ("std::vector<std::vector<cv::Vec2d>>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_Vec2dGG_delete(std::vector<std::vector<cv::Vec2d>>* instance) {
			delete instance;
	}

	// std::vector<std::vector<cv::Vec2d>>::len() generated
	// ("std::vector<std::vector<cv::Vec2d>>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_vectorLcv_Vec2dGG_len_const(const std::vector<std::vector<cv::Vec2d>>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<std::vector<cv::Vec2d>>::isEmpty() generated
	// ("std::vector<std::vector<cv::Vec2d>>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLstd_vectorLcv_Vec2dGG_isEmpty_const(const std::vector<std::vector<cv::Vec2d>>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<std::vector<cv::Vec2d>>::capacity() generated
	// ("std::vector<std::vector<cv::Vec2d>>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_vectorLcv_Vec2dGG_capacity_const(const std::vector<std::vector<cv::Vec2d>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<std::vector<cv::Vec2d>>::shrinkToFit() generated
	// ("std::vector<std::vector<cv::Vec2d>>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_Vec2dGG_shrinkToFit(std::vector<std::vector<cv::Vec2d>>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<std::vector<cv::Vec2d>>::reserve(Primitive) generated
	// ("std::vector<std::vector<cv::Vec2d>>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLcv_Vec2dGG_reserve_size_t(std::vector<std::vector<cv::Vec2d>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<std::vector<cv::Vec2d>>::remove(Primitive) generated
	// ("std::vector<std::vector<cv::Vec2d>>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLcv_Vec2dGG_remove_size_t(std::vector<std::vector<cv::Vec2d>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<std::vector<cv::Vec2d>>::swap(Primitive, Primitive) generated
	// ("std::vector<std::vector<cv::Vec2d>>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLstd_vectorLcv_Vec2dGG_swap_size_t_size_t(std::vector<std::vector<cv::Vec2d>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<std::vector<cv::Vec2d>>::clear() generated
	// ("std::vector<std::vector<cv::Vec2d>>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_Vec2dGG_clear(std::vector<std::vector<cv::Vec2d>>* instance) {
			instance->clear();
	}

	// std::vector<std::vector<cv::Vec2d>>::push(CppPassByVoidPtr) generated
	// ("std::vector<std::vector<cv::Vec2d>>::push", vec![(pred!(mut, ["val"], ["const std::vector<cv::Vec2d>"]), _)]),
	void std_vectorLstd_vectorLcv_Vec2dGG_push_const_vectorLVec2dG(std::vector<std::vector<cv::Vec2d>>* instance, const std::vector<cv::Vec2d>* val) {
			instance->push_back(*val);
	}

	// std::vector<std::vector<cv::Vec2d>>::insert(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::vector<cv::Vec2d>>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::vector<cv::Vec2d>"]), _)]),
	void std_vectorLstd_vectorLcv_Vec2dGG_insert_size_t_const_vectorLVec2dG(std::vector<std::vector<cv::Vec2d>>* instance, size_t index, const std::vector<cv::Vec2d>* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<std::vector<cv::Vec2d>>::get(Primitive) generated
	// ("std::vector<std::vector<cv::Vec2d>>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLcv_Vec2dGG_get_const_size_t(const std::vector<std::vector<cv::Vec2d>>* instance, size_t index, std::vector<cv::Vec2d>** ocvrs_return) {
			std::vector<cv::Vec2d> ret = (*instance)[index];
			*ocvrs_return = new std::vector<cv::Vec2d>(ret);
	}

	// std::vector<std::vector<cv::Vec2d>>::set(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::vector<cv::Vec2d>>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::vector<cv::Vec2d>"]), _)]),
	void std_vectorLstd_vectorLcv_Vec2dGG_set_size_t_const_vectorLVec2dG(std::vector<std::vector<cv::Vec2d>>* instance, size_t index, const std::vector<cv::Vec2d>* val) {
			(*instance)[index] = *val;
	}

	// std::vector<std::vector<cv::Vec2d>>::inputArray() generated
	// ("std::vector<std::vector<cv::Vec2d>>::inputArray", vec![(pred!(const, [], []), _)]),
	void std_vectorLstd_vectorLcv_Vec2dGG_inputArray_const(const std::vector<std::vector<cv::Vec2d>>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			const cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new const cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<std::vector<cv::Vec2d>>::outputArray() generated
	// ("std::vector<std::vector<cv::Vec2d>>::outputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_Vec2dGG_outputArray(std::vector<std::vector<cv::Vec2d>>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<std::vector<cv::Vec2d>>::inputOutputArray() generated
	// ("std::vector<std::vector<cv::Vec2d>>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_Vec2dGG_inputOutputArray(std::vector<std::vector<cv::Vec2d>>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

}


