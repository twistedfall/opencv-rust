extern "C" {
	// std::vector<cvflann::lsh::FeatureIndex>::new() generated
	// ("std::vector<cvflann::lsh::FeatureIndex>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cvflann::lsh::FeatureIndex>* std_vectorLcvflann_lsh_FeatureIndexG_new_const() {
			std::vector<cvflann::lsh::FeatureIndex>* ret = new std::vector<cvflann::lsh::FeatureIndex>();
			return ret;
	}

	// std::vector<cvflann::lsh::FeatureIndex>::delete() generated
	// ("std::vector<cvflann::lsh::FeatureIndex>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcvflann_lsh_FeatureIndexG_delete(std::vector<cvflann::lsh::FeatureIndex>* instance) {
			delete instance;
	}

	// std::vector<cvflann::lsh::FeatureIndex>::len() generated
	// ("std::vector<cvflann::lsh::FeatureIndex>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcvflann_lsh_FeatureIndexG_len_const(const std::vector<cvflann::lsh::FeatureIndex>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cvflann::lsh::FeatureIndex>::isEmpty() generated
	// ("std::vector<cvflann::lsh::FeatureIndex>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcvflann_lsh_FeatureIndexG_isEmpty_const(const std::vector<cvflann::lsh::FeatureIndex>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cvflann::lsh::FeatureIndex>::capacity() generated
	// ("std::vector<cvflann::lsh::FeatureIndex>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcvflann_lsh_FeatureIndexG_capacity_const(const std::vector<cvflann::lsh::FeatureIndex>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cvflann::lsh::FeatureIndex>::shrinkToFit() generated
	// ("std::vector<cvflann::lsh::FeatureIndex>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcvflann_lsh_FeatureIndexG_shrinkToFit(std::vector<cvflann::lsh::FeatureIndex>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cvflann::lsh::FeatureIndex>::reserve(Primitive) generated
	// ("std::vector<cvflann::lsh::FeatureIndex>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcvflann_lsh_FeatureIndexG_reserve_size_t(std::vector<cvflann::lsh::FeatureIndex>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cvflann::lsh::FeatureIndex>::remove(Primitive) generated
	// ("std::vector<cvflann::lsh::FeatureIndex>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcvflann_lsh_FeatureIndexG_remove_size_t(std::vector<cvflann::lsh::FeatureIndex>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cvflann::lsh::FeatureIndex>::swap(Primitive, Primitive) generated
	// ("std::vector<cvflann::lsh::FeatureIndex>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcvflann_lsh_FeatureIndexG_swap_size_t_size_t(std::vector<cvflann::lsh::FeatureIndex>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cvflann::lsh::FeatureIndex>::clear() generated
	// ("std::vector<cvflann::lsh::FeatureIndex>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcvflann_lsh_FeatureIndexG_clear(std::vector<cvflann::lsh::FeatureIndex>* instance) {
			instance->clear();
	}

	// std::vector<cvflann::lsh::FeatureIndex>::push(Primitive) generated
	// ("std::vector<cvflann::lsh::FeatureIndex>::push", vec![(pred!(mut, ["val"], ["const cvflann::lsh::FeatureIndex"]), _)]),
	void std_vectorLcvflann_lsh_FeatureIndexG_push_const_FeatureIndex(std::vector<cvflann::lsh::FeatureIndex>* instance, const cvflann::lsh::FeatureIndex val) {
			instance->push_back(val);
	}

	// std::vector<cvflann::lsh::FeatureIndex>::insert(Primitive, Primitive) generated
	// ("std::vector<cvflann::lsh::FeatureIndex>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cvflann::lsh::FeatureIndex"]), _)]),
	void std_vectorLcvflann_lsh_FeatureIndexG_insert_size_t_const_FeatureIndex(std::vector<cvflann::lsh::FeatureIndex>* instance, size_t index, const cvflann::lsh::FeatureIndex val) {
			instance->insert(instance->begin() + index, val);
	}

	// std::vector<cvflann::lsh::FeatureIndex>::get(Primitive) generated
	// ("std::vector<cvflann::lsh::FeatureIndex>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcvflann_lsh_FeatureIndexG_get_const_size_t(const std::vector<cvflann::lsh::FeatureIndex>* instance, size_t index, cvflann::lsh::FeatureIndex* ocvrs_return) {
			cvflann::lsh::FeatureIndex ret = (*instance)[index];
			*ocvrs_return = ret;
	}

	// std::vector<cvflann::lsh::FeatureIndex>::set(Primitive, Primitive) generated
	// ("std::vector<cvflann::lsh::FeatureIndex>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cvflann::lsh::FeatureIndex"]), _)]),
	void std_vectorLcvflann_lsh_FeatureIndexG_set_size_t_const_FeatureIndex(std::vector<cvflann::lsh::FeatureIndex>* instance, size_t index, const cvflann::lsh::FeatureIndex val) {
			(*instance)[index] = val;
	}

	// std::vector<cvflann::lsh::FeatureIndex>::clone() generated
	// ("std::vector<cvflann::lsh::FeatureIndex>::clone", vec![(pred!(const, [], []), _)]),
	std::vector<cvflann::lsh::FeatureIndex>* std_vectorLcvflann_lsh_FeatureIndexG_clone_const(const std::vector<cvflann::lsh::FeatureIndex>* instance) {
			std::vector<cvflann::lsh::FeatureIndex> ret = std::vector<cvflann::lsh::FeatureIndex>(*instance);
			return new std::vector<cvflann::lsh::FeatureIndex>(ret);
	}

	// std::vector<cvflann::lsh::FeatureIndex>::data() generated
	// ("std::vector<cvflann::lsh::FeatureIndex>::data", vec![(pred!(const, [], []), _)]),
	const cvflann::lsh::FeatureIndex* std_vectorLcvflann_lsh_FeatureIndexG_data_const(const std::vector<cvflann::lsh::FeatureIndex>* instance) {
			const cvflann::lsh::FeatureIndex* ret = instance->data();
			return ret;
	}

	// std::vector<cvflann::lsh::FeatureIndex>::dataMut() generated
	// ("std::vector<cvflann::lsh::FeatureIndex>::dataMut", vec![(pred!(mut, [], []), _)]),
	cvflann::lsh::FeatureIndex* std_vectorLcvflann_lsh_FeatureIndexG_dataMut(std::vector<cvflann::lsh::FeatureIndex>* instance) {
			cvflann::lsh::FeatureIndex* ret = instance->data();
			return ret;
	}

	// cv::fromSlice(Indirect, Primitive) generated
	// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const cvflann::lsh::FeatureIndex*", "size_t"]), _)]),
	std::vector<cvflann::lsh::FeatureIndex>* cv_fromSlice_const_const_FeatureIndexX_size_t(const cvflann::lsh::FeatureIndex* data, size_t len) {
			return new std::vector<cvflann::lsh::FeatureIndex>(data, data + len);
	}

	#if CV_VERSION_MAJOR == 5
	// std::vector<cvflann::lsh::FeatureIndex>::inputArray() generated
	// ("std::vector<cvflann::lsh::FeatureIndex>::inputArray", vec![(pred!(const, [], []), _)]),
	void std_vectorLcvflann_lsh_FeatureIndexG_inputArray_const(const std::vector<cvflann::lsh::FeatureIndex>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			const cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new const cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	#endif

	#if CV_VERSION_MAJOR == 5
	// std::vector<cvflann::lsh::FeatureIndex>::outputArray() generated
	// ("std::vector<cvflann::lsh::FeatureIndex>::outputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcvflann_lsh_FeatureIndexG_outputArray(std::vector<cvflann::lsh::FeatureIndex>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	#endif

	#if CV_VERSION_MAJOR == 5
	// std::vector<cvflann::lsh::FeatureIndex>::inputOutputArray() generated
	// ("std::vector<cvflann::lsh::FeatureIndex>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcvflann_lsh_FeatureIndexG_inputOutputArray(std::vector<cvflann::lsh::FeatureIndex>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	#endif

}


