extern "C" {
	// std::vector<std::vector<cv::Vec3d>>::new() generated
	// ("std::vector<std::vector<cv::Vec3d>>::new", vec![(pred!(const, [], []), _)]),
	std::vector<std::vector<cv::Vec3d>>* std_vectorLstd_vectorLcv_Vec3dGG_new_const() {
			std::vector<std::vector<cv::Vec3d>>* ret = new std::vector<std::vector<cv::Vec3d>>();
			return ret;
	}

	// std::vector<std::vector<cv::Vec3d>>::delete() generated
	// ("std::vector<std::vector<cv::Vec3d>>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_Vec3dGG_delete(std::vector<std::vector<cv::Vec3d>>* instance) {
			delete instance;
	}

	// std::vector<std::vector<cv::Vec3d>>::len() generated
	// ("std::vector<std::vector<cv::Vec3d>>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_vectorLcv_Vec3dGG_len_const(const std::vector<std::vector<cv::Vec3d>>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<std::vector<cv::Vec3d>>::isEmpty() generated
	// ("std::vector<std::vector<cv::Vec3d>>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLstd_vectorLcv_Vec3dGG_isEmpty_const(const std::vector<std::vector<cv::Vec3d>>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<std::vector<cv::Vec3d>>::capacity() generated
	// ("std::vector<std::vector<cv::Vec3d>>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_vectorLcv_Vec3dGG_capacity_const(const std::vector<std::vector<cv::Vec3d>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<std::vector<cv::Vec3d>>::shrinkToFit() generated
	// ("std::vector<std::vector<cv::Vec3d>>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_Vec3dGG_shrinkToFit(std::vector<std::vector<cv::Vec3d>>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<std::vector<cv::Vec3d>>::reserve(Primitive) generated
	// ("std::vector<std::vector<cv::Vec3d>>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLcv_Vec3dGG_reserve_size_t(std::vector<std::vector<cv::Vec3d>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<std::vector<cv::Vec3d>>::remove(Primitive) generated
	// ("std::vector<std::vector<cv::Vec3d>>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLcv_Vec3dGG_remove_size_t(std::vector<std::vector<cv::Vec3d>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<std::vector<cv::Vec3d>>::swap(Primitive, Primitive) generated
	// ("std::vector<std::vector<cv::Vec3d>>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLstd_vectorLcv_Vec3dGG_swap_size_t_size_t(std::vector<std::vector<cv::Vec3d>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<std::vector<cv::Vec3d>>::clear() generated
	// ("std::vector<std::vector<cv::Vec3d>>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_Vec3dGG_clear(std::vector<std::vector<cv::Vec3d>>* instance) {
			instance->clear();
	}

	// std::vector<std::vector<cv::Vec3d>>::push(CppPassByVoidPtr) generated
	// ("std::vector<std::vector<cv::Vec3d>>::push", vec![(pred!(mut, ["val"], ["const std::vector<cv::Vec3d>"]), _)]),
	void std_vectorLstd_vectorLcv_Vec3dGG_push_const_vectorLVec3dG(std::vector<std::vector<cv::Vec3d>>* instance, const std::vector<cv::Vec3d>* val) {
			instance->push_back(*val);
	}

	// std::vector<std::vector<cv::Vec3d>>::insert(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::vector<cv::Vec3d>>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::vector<cv::Vec3d>"]), _)]),
	void std_vectorLstd_vectorLcv_Vec3dGG_insert_size_t_const_vectorLVec3dG(std::vector<std::vector<cv::Vec3d>>* instance, size_t index, const std::vector<cv::Vec3d>* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<std::vector<cv::Vec3d>>::get(Primitive) generated
	// ("std::vector<std::vector<cv::Vec3d>>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLcv_Vec3dGG_get_const_size_t(const std::vector<std::vector<cv::Vec3d>>* instance, size_t index, std::vector<cv::Vec3d>** ocvrs_return) {
			std::vector<cv::Vec3d> ret = (*instance)[index];
			*ocvrs_return = new std::vector<cv::Vec3d>(ret);
	}

	// std::vector<std::vector<cv::Vec3d>>::set(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::vector<cv::Vec3d>>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::vector<cv::Vec3d>"]), _)]),
	void std_vectorLstd_vectorLcv_Vec3dGG_set_size_t_const_vectorLVec3dG(std::vector<std::vector<cv::Vec3d>>* instance, size_t index, const std::vector<cv::Vec3d>* val) {
			(*instance)[index] = *val;
	}

	// std::vector<std::vector<cv::Vec3d>>::inputArray() generated
	// ("std::vector<std::vector<cv::Vec3d>>::inputArray", vec![(pred!(const, [], []), _)]),
	void std_vectorLstd_vectorLcv_Vec3dGG_inputArray_const(const std::vector<std::vector<cv::Vec3d>>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			const cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new const cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<std::vector<cv::Vec3d>>::outputArray() generated
	// ("std::vector<std::vector<cv::Vec3d>>::outputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_Vec3dGG_outputArray(std::vector<std::vector<cv::Vec3d>>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<std::vector<cv::Vec3d>>::inputOutputArray() generated
	// ("std::vector<std::vector<cv::Vec3d>>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_Vec3dGG_inputOutputArray(std::vector<std::vector<cv::Vec3d>>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

}


