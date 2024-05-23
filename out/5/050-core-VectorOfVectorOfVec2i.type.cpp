extern "C" {
	// std::vector<std::vector<cv::Vec2i>>::new() generated
	// ("std::vector<std::vector<cv::Vec2i>>::new", vec![(pred!(const, [], []), _)]),
	std::vector<std::vector<cv::Vec2i>>* std_vectorLstd_vectorLcv_Vec2iGG_new_const() {
			std::vector<std::vector<cv::Vec2i>>* ret = new std::vector<std::vector<cv::Vec2i>>();
			return ret;
	}

	// std::vector<std::vector<cv::Vec2i>>::delete() generated
	// ("std::vector<std::vector<cv::Vec2i>>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_Vec2iGG_delete(std::vector<std::vector<cv::Vec2i>>* instance) {
			delete instance;
	}

	// std::vector<std::vector<cv::Vec2i>>::len() generated
	// ("std::vector<std::vector<cv::Vec2i>>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_vectorLcv_Vec2iGG_len_const(const std::vector<std::vector<cv::Vec2i>>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<std::vector<cv::Vec2i>>::isEmpty() generated
	// ("std::vector<std::vector<cv::Vec2i>>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLstd_vectorLcv_Vec2iGG_isEmpty_const(const std::vector<std::vector<cv::Vec2i>>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<std::vector<cv::Vec2i>>::capacity() generated
	// ("std::vector<std::vector<cv::Vec2i>>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_vectorLcv_Vec2iGG_capacity_const(const std::vector<std::vector<cv::Vec2i>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<std::vector<cv::Vec2i>>::shrinkToFit() generated
	// ("std::vector<std::vector<cv::Vec2i>>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_Vec2iGG_shrinkToFit(std::vector<std::vector<cv::Vec2i>>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<std::vector<cv::Vec2i>>::reserve(Primitive) generated
	// ("std::vector<std::vector<cv::Vec2i>>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLcv_Vec2iGG_reserve_size_t(std::vector<std::vector<cv::Vec2i>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<std::vector<cv::Vec2i>>::remove(Primitive) generated
	// ("std::vector<std::vector<cv::Vec2i>>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLcv_Vec2iGG_remove_size_t(std::vector<std::vector<cv::Vec2i>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<std::vector<cv::Vec2i>>::swap(Primitive, Primitive) generated
	// ("std::vector<std::vector<cv::Vec2i>>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLstd_vectorLcv_Vec2iGG_swap_size_t_size_t(std::vector<std::vector<cv::Vec2i>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<std::vector<cv::Vec2i>>::clear() generated
	// ("std::vector<std::vector<cv::Vec2i>>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_Vec2iGG_clear(std::vector<std::vector<cv::Vec2i>>* instance) {
			instance->clear();
	}

	// std::vector<std::vector<cv::Vec2i>>::push(CppPassByVoidPtr) generated
	// ("std::vector<std::vector<cv::Vec2i>>::push", vec![(pred!(mut, ["val"], ["const std::vector<cv::Vec2i>"]), _)]),
	void std_vectorLstd_vectorLcv_Vec2iGG_push_const_vectorLVec2iG(std::vector<std::vector<cv::Vec2i>>* instance, const std::vector<cv::Vec2i>* val) {
			instance->push_back(*val);
	}

	// std::vector<std::vector<cv::Vec2i>>::insert(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::vector<cv::Vec2i>>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::vector<cv::Vec2i>"]), _)]),
	void std_vectorLstd_vectorLcv_Vec2iGG_insert_size_t_const_vectorLVec2iG(std::vector<std::vector<cv::Vec2i>>* instance, size_t index, const std::vector<cv::Vec2i>* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<std::vector<cv::Vec2i>>::get(Primitive) generated
	// ("std::vector<std::vector<cv::Vec2i>>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLcv_Vec2iGG_get_const_size_t(const std::vector<std::vector<cv::Vec2i>>* instance, size_t index, std::vector<cv::Vec2i>** ocvrs_return) {
			std::vector<cv::Vec2i> ret = (*instance)[index];
			*ocvrs_return = new std::vector<cv::Vec2i>(ret);
	}

	// std::vector<std::vector<cv::Vec2i>>::set(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::vector<cv::Vec2i>>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::vector<cv::Vec2i>"]), _)]),
	void std_vectorLstd_vectorLcv_Vec2iGG_set_size_t_const_vectorLVec2iG(std::vector<std::vector<cv::Vec2i>>* instance, size_t index, const std::vector<cv::Vec2i>* val) {
			(*instance)[index] = *val;
	}

	// std::vector<std::vector<cv::Vec2i>>::inputArray() generated
	// ("std::vector<std::vector<cv::Vec2i>>::inputArray", vec![(pred!(const, [], []), _)]),
	void std_vectorLstd_vectorLcv_Vec2iGG_inputArray_const(const std::vector<std::vector<cv::Vec2i>>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			const cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new const cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<std::vector<cv::Vec2i>>::outputArray() generated
	// ("std::vector<std::vector<cv::Vec2i>>::outputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_Vec2iGG_outputArray(std::vector<std::vector<cv::Vec2i>>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<std::vector<cv::Vec2i>>::inputOutputArray() generated
	// ("std::vector<std::vector<cv::Vec2i>>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_Vec2iGG_inputOutputArray(std::vector<std::vector<cv::Vec2i>>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

}


