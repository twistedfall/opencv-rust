extern "C" {
	// std::vector<std::vector<cv::Point>>::new() generated
	// ("std::vector<std::vector<cv::Point>>::new", vec![(pred!(const, [], []), _)]),
	std::vector<std::vector<cv::Point>>* std_vectorLstd_vectorLcv_PointGG_new_const() {
			std::vector<std::vector<cv::Point>>* ret = new std::vector<std::vector<cv::Point>>();
			return ret;
	}

	// std::vector<std::vector<cv::Point>>::delete() generated
	// ("std::vector<std::vector<cv::Point>>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_PointGG_delete(std::vector<std::vector<cv::Point>>* instance) {
			delete instance;
	}

	// std::vector<std::vector<cv::Point>>::len() generated
	// ("std::vector<std::vector<cv::Point>>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_vectorLcv_PointGG_len_const(const std::vector<std::vector<cv::Point>>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<std::vector<cv::Point>>::isEmpty() generated
	// ("std::vector<std::vector<cv::Point>>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLstd_vectorLcv_PointGG_isEmpty_const(const std::vector<std::vector<cv::Point>>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<std::vector<cv::Point>>::capacity() generated
	// ("std::vector<std::vector<cv::Point>>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_vectorLcv_PointGG_capacity_const(const std::vector<std::vector<cv::Point>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<std::vector<cv::Point>>::shrinkToFit() generated
	// ("std::vector<std::vector<cv::Point>>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_PointGG_shrinkToFit(std::vector<std::vector<cv::Point>>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<std::vector<cv::Point>>::reserve(Primitive) generated
	// ("std::vector<std::vector<cv::Point>>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLcv_PointGG_reserve_size_t(std::vector<std::vector<cv::Point>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<std::vector<cv::Point>>::remove(Primitive) generated
	// ("std::vector<std::vector<cv::Point>>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLcv_PointGG_remove_size_t(std::vector<std::vector<cv::Point>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<std::vector<cv::Point>>::swap(Primitive, Primitive) generated
	// ("std::vector<std::vector<cv::Point>>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLstd_vectorLcv_PointGG_swap_size_t_size_t(std::vector<std::vector<cv::Point>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<std::vector<cv::Point>>::clear() generated
	// ("std::vector<std::vector<cv::Point>>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_PointGG_clear(std::vector<std::vector<cv::Point>>* instance) {
			instance->clear();
	}

	// std::vector<std::vector<cv::Point>>::push(CppPassByVoidPtr) generated
	// ("std::vector<std::vector<cv::Point>>::push", vec![(pred!(mut, ["val"], ["const std::vector<cv::Point>"]), _)]),
	void std_vectorLstd_vectorLcv_PointGG_push_const_vectorLPointG(std::vector<std::vector<cv::Point>>* instance, const std::vector<cv::Point>* val) {
			instance->push_back(*val);
	}

	// std::vector<std::vector<cv::Point>>::insert(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::vector<cv::Point>>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::vector<cv::Point>"]), _)]),
	void std_vectorLstd_vectorLcv_PointGG_insert_size_t_const_vectorLPointG(std::vector<std::vector<cv::Point>>* instance, size_t index, const std::vector<cv::Point>* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<std::vector<cv::Point>>::get(Primitive) generated
	// ("std::vector<std::vector<cv::Point>>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLcv_PointGG_get_const_size_t(const std::vector<std::vector<cv::Point>>* instance, size_t index, std::vector<cv::Point>** ocvrs_return) {
			std::vector<cv::Point> ret = (*instance)[index];
			*ocvrs_return = new std::vector<cv::Point>(ret);
	}

	// std::vector<std::vector<cv::Point>>::set(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::vector<cv::Point>>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::vector<cv::Point>"]), _)]),
	void std_vectorLstd_vectorLcv_PointGG_set_size_t_const_vectorLPointG(std::vector<std::vector<cv::Point>>* instance, size_t index, const std::vector<cv::Point>* val) {
			(*instance)[index] = *val;
	}

	// std::vector<std::vector<cv::Point>>::inputArray() generated
	// ("std::vector<std::vector<cv::Point>>::inputArray", vec![(pred!(const, [], []), _)]),
	void std_vectorLstd_vectorLcv_PointGG_inputArray_const(const std::vector<std::vector<cv::Point>>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			const cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new const cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<std::vector<cv::Point>>::outputArray() generated
	// ("std::vector<std::vector<cv::Point>>::outputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_PointGG_outputArray(std::vector<std::vector<cv::Point>>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<std::vector<cv::Point>>::inputOutputArray() generated
	// ("std::vector<std::vector<cv::Point>>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_PointGG_inputOutputArray(std::vector<std::vector<cv::Point>>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

}


