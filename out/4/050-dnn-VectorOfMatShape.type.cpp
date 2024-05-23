extern "C" {
	// std::vector<cv::dnn::MatShape>::new() generated
	// ("std::vector<cv::dnn::MatShape>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::dnn::MatShape>* std_vectorLcv_dnn_MatShapeG_new_const() {
			std::vector<cv::dnn::MatShape>* ret = new std::vector<cv::dnn::MatShape>();
			return ret;
	}

	// std::vector<cv::dnn::MatShape>::delete() generated
	// ("std::vector<cv::dnn::MatShape>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_dnn_MatShapeG_delete(std::vector<cv::dnn::MatShape>* instance) {
			delete instance;
	}

	// std::vector<cv::dnn::MatShape>::len() generated
	// ("std::vector<cv::dnn::MatShape>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_dnn_MatShapeG_len_const(const std::vector<cv::dnn::MatShape>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::dnn::MatShape>::isEmpty() generated
	// ("std::vector<cv::dnn::MatShape>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_dnn_MatShapeG_isEmpty_const(const std::vector<cv::dnn::MatShape>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::dnn::MatShape>::capacity() generated
	// ("std::vector<cv::dnn::MatShape>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_dnn_MatShapeG_capacity_const(const std::vector<cv::dnn::MatShape>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::dnn::MatShape>::shrinkToFit() generated
	// ("std::vector<cv::dnn::MatShape>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_dnn_MatShapeG_shrinkToFit(std::vector<cv::dnn::MatShape>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::dnn::MatShape>::reserve(Primitive) generated
	// ("std::vector<cv::dnn::MatShape>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_dnn_MatShapeG_reserve_size_t(std::vector<cv::dnn::MatShape>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::dnn::MatShape>::remove(Primitive) generated
	// ("std::vector<cv::dnn::MatShape>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_dnn_MatShapeG_remove_size_t(std::vector<cv::dnn::MatShape>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::dnn::MatShape>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::dnn::MatShape>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_dnn_MatShapeG_swap_size_t_size_t(std::vector<cv::dnn::MatShape>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::dnn::MatShape>::clear() generated
	// ("std::vector<cv::dnn::MatShape>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_dnn_MatShapeG_clear(std::vector<cv::dnn::MatShape>* instance) {
			instance->clear();
	}

	// std::vector<cv::dnn::MatShape>::push(CppPassByVoidPtr) generated
	// ("std::vector<cv::dnn::MatShape>::push", vec![(pred!(mut, ["val"], ["const cv::dnn::MatShape"]), _)]),
	void std_vectorLcv_dnn_MatShapeG_push_const_MatShape(std::vector<cv::dnn::MatShape>* instance, const cv::dnn::MatShape* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::dnn::MatShape>::insert(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<cv::dnn::MatShape>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::dnn::MatShape"]), _)]),
	void std_vectorLcv_dnn_MatShapeG_insert_size_t_const_MatShape(std::vector<cv::dnn::MatShape>* instance, size_t index, const cv::dnn::MatShape* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::dnn::MatShape>::get(Primitive) generated
	// ("std::vector<cv::dnn::MatShape>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_dnn_MatShapeG_get_const_size_t(const std::vector<cv::dnn::MatShape>* instance, size_t index, cv::dnn::MatShape** ocvrs_return) {
			cv::dnn::MatShape ret = (*instance)[index];
			*ocvrs_return = new cv::dnn::MatShape(ret);
	}

	// std::vector<cv::dnn::MatShape>::set(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<cv::dnn::MatShape>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::dnn::MatShape"]), _)]),
	void std_vectorLcv_dnn_MatShapeG_set_size_t_const_MatShape(std::vector<cv::dnn::MatShape>* instance, size_t index, const cv::dnn::MatShape* val) {
			(*instance)[index] = *val;
	}

	// std::vector<cv::dnn::MatShape>::inputArray() generated
	// ("std::vector<cv::dnn::MatShape>::inputArray", vec![(pred!(const, [], []), _)]),
	void std_vectorLcv_dnn_MatShapeG_inputArray_const(const std::vector<cv::dnn::MatShape>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			const cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new const cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<cv::dnn::MatShape>::outputArray() generated
	// ("std::vector<cv::dnn::MatShape>::outputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_dnn_MatShapeG_outputArray(std::vector<cv::dnn::MatShape>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// std::vector<cv::dnn::MatShape>::inputOutputArray() generated
	// ("std::vector<cv::dnn::MatShape>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_dnn_MatShapeG_inputOutputArray(std::vector<cv::dnn::MatShape>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

}


