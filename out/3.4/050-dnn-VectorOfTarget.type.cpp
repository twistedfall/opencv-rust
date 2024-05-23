extern "C" {
	// std::vector<cv::dnn::Target>::new() generated
	// ("std::vector<cv::dnn::Target>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::dnn::Target>* std_vectorLcv_dnn_TargetG_new_const() {
			std::vector<cv::dnn::Target>* ret = new std::vector<cv::dnn::Target>();
			return ret;
	}

	// std::vector<cv::dnn::Target>::delete() generated
	// ("std::vector<cv::dnn::Target>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_dnn_TargetG_delete(std::vector<cv::dnn::Target>* instance) {
			delete instance;
	}

	// std::vector<cv::dnn::Target>::len() generated
	// ("std::vector<cv::dnn::Target>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_dnn_TargetG_len_const(const std::vector<cv::dnn::Target>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::dnn::Target>::isEmpty() generated
	// ("std::vector<cv::dnn::Target>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_dnn_TargetG_isEmpty_const(const std::vector<cv::dnn::Target>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::dnn::Target>::capacity() generated
	// ("std::vector<cv::dnn::Target>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_dnn_TargetG_capacity_const(const std::vector<cv::dnn::Target>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::dnn::Target>::shrinkToFit() generated
	// ("std::vector<cv::dnn::Target>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_dnn_TargetG_shrinkToFit(std::vector<cv::dnn::Target>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::dnn::Target>::reserve(Primitive) generated
	// ("std::vector<cv::dnn::Target>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_dnn_TargetG_reserve_size_t(std::vector<cv::dnn::Target>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::dnn::Target>::remove(Primitive) generated
	// ("std::vector<cv::dnn::Target>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_dnn_TargetG_remove_size_t(std::vector<cv::dnn::Target>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::dnn::Target>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::dnn::Target>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_dnn_TargetG_swap_size_t_size_t(std::vector<cv::dnn::Target>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::dnn::Target>::clear() generated
	// ("std::vector<cv::dnn::Target>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_dnn_TargetG_clear(std::vector<cv::dnn::Target>* instance) {
			instance->clear();
	}

	// std::vector<cv::dnn::Target>::push(Enum) generated
	// ("std::vector<cv::dnn::Target>::push", vec![(pred!(mut, ["val"], ["const cv::dnn::Target"]), _)]),
	void std_vectorLcv_dnn_TargetG_push_const_Target(std::vector<cv::dnn::Target>* instance, const cv::dnn::Target val) {
			instance->push_back(val);
	}

	// std::vector<cv::dnn::Target>::insert(Primitive, Enum) generated
	// ("std::vector<cv::dnn::Target>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::dnn::Target"]), _)]),
	void std_vectorLcv_dnn_TargetG_insert_size_t_const_Target(std::vector<cv::dnn::Target>* instance, size_t index, const cv::dnn::Target val) {
			instance->insert(instance->begin() + index, val);
	}

	// std::vector<cv::dnn::Target>::get(Primitive) generated
	// ("std::vector<cv::dnn::Target>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_dnn_TargetG_get_const_size_t(const std::vector<cv::dnn::Target>* instance, size_t index, cv::dnn::Target* ocvrs_return) {
			cv::dnn::Target ret = (*instance)[index];
			*ocvrs_return = ret;
	}

	// std::vector<cv::dnn::Target>::set(Primitive, Enum) generated
	// ("std::vector<cv::dnn::Target>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::dnn::Target"]), _)]),
	void std_vectorLcv_dnn_TargetG_set_size_t_const_Target(std::vector<cv::dnn::Target>* instance, size_t index, const cv::dnn::Target val) {
			(*instance)[index] = val;
	}

	// std::vector<cv::dnn::Target>::clone() generated
	// ("std::vector<cv::dnn::Target>::clone", vec![(pred!(const, [], []), _)]),
	std::vector<cv::dnn::Target>* std_vectorLcv_dnn_TargetG_clone_const(const std::vector<cv::dnn::Target>* instance) {
			std::vector<cv::dnn::Target> ret = std::vector<cv::dnn::Target>(*instance);
			return new std::vector<cv::dnn::Target>(ret);
	}

	// std::vector<cv::dnn::Target>::data() generated
	// ("std::vector<cv::dnn::Target>::data", vec![(pred!(const, [], []), _)]),
	const cv::dnn::Target* std_vectorLcv_dnn_TargetG_data_const(const std::vector<cv::dnn::Target>* instance) {
			const cv::dnn::Target* ret = instance->data();
			return ret;
	}

	// std::vector<cv::dnn::Target>::dataMut() generated
	// ("std::vector<cv::dnn::Target>::dataMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Target* std_vectorLcv_dnn_TargetG_dataMut(std::vector<cv::dnn::Target>* instance) {
			cv::dnn::Target* ret = instance->data();
			return ret;
	}

	// cv::fromSlice(Enum, Primitive) generated
	// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const cv::dnn::Target*", "size_t"]), _)]),
	std::vector<cv::dnn::Target>* cv_fromSlice_const_const_TargetX_size_t(const cv::dnn::Target* data, size_t len) {
			return new std::vector<cv::dnn::Target>(data, data + len);
	}

}


