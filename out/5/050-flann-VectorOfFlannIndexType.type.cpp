extern "C" {
	// std::vector<cv::flann::FlannIndexType>::new() generated
	// ("std::vector<cv::flann::FlannIndexType>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::flann::FlannIndexType>* std_vectorLcv_flann_FlannIndexTypeG_new_const() {
			std::vector<cv::flann::FlannIndexType>* ret = new std::vector<cv::flann::FlannIndexType>();
			return ret;
	}

	// std::vector<cv::flann::FlannIndexType>::delete() generated
	// ("std::vector<cv::flann::FlannIndexType>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_flann_FlannIndexTypeG_delete(std::vector<cv::flann::FlannIndexType>* instance) {
			delete instance;
	}

	// std::vector<cv::flann::FlannIndexType>::len() generated
	// ("std::vector<cv::flann::FlannIndexType>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_flann_FlannIndexTypeG_len_const(const std::vector<cv::flann::FlannIndexType>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::flann::FlannIndexType>::isEmpty() generated
	// ("std::vector<cv::flann::FlannIndexType>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_flann_FlannIndexTypeG_isEmpty_const(const std::vector<cv::flann::FlannIndexType>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::flann::FlannIndexType>::capacity() generated
	// ("std::vector<cv::flann::FlannIndexType>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_flann_FlannIndexTypeG_capacity_const(const std::vector<cv::flann::FlannIndexType>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::flann::FlannIndexType>::shrinkToFit() generated
	// ("std::vector<cv::flann::FlannIndexType>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_flann_FlannIndexTypeG_shrinkToFit(std::vector<cv::flann::FlannIndexType>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::flann::FlannIndexType>::reserve(Primitive) generated
	// ("std::vector<cv::flann::FlannIndexType>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_flann_FlannIndexTypeG_reserve_size_t(std::vector<cv::flann::FlannIndexType>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::flann::FlannIndexType>::remove(Primitive) generated
	// ("std::vector<cv::flann::FlannIndexType>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_flann_FlannIndexTypeG_remove_size_t(std::vector<cv::flann::FlannIndexType>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::flann::FlannIndexType>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::flann::FlannIndexType>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_flann_FlannIndexTypeG_swap_size_t_size_t(std::vector<cv::flann::FlannIndexType>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::flann::FlannIndexType>::clear() generated
	// ("std::vector<cv::flann::FlannIndexType>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_flann_FlannIndexTypeG_clear(std::vector<cv::flann::FlannIndexType>* instance) {
			instance->clear();
	}

	// std::vector<cv::flann::FlannIndexType>::push(Enum) generated
	// ("std::vector<cv::flann::FlannIndexType>::push", vec![(pred!(mut, ["val"], ["const cv::flann::FlannIndexType"]), _)]),
	void std_vectorLcv_flann_FlannIndexTypeG_push_const_FlannIndexType(std::vector<cv::flann::FlannIndexType>* instance, const cv::flann::FlannIndexType val) {
			instance->push_back(val);
	}

	// std::vector<cv::flann::FlannIndexType>::insert(Primitive, Enum) generated
	// ("std::vector<cv::flann::FlannIndexType>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::flann::FlannIndexType"]), _)]),
	void std_vectorLcv_flann_FlannIndexTypeG_insert_size_t_const_FlannIndexType(std::vector<cv::flann::FlannIndexType>* instance, size_t index, const cv::flann::FlannIndexType val) {
			instance->insert(instance->begin() + index, val);
	}

	// std::vector<cv::flann::FlannIndexType>::get(Primitive) generated
	// ("std::vector<cv::flann::FlannIndexType>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_flann_FlannIndexTypeG_get_const_size_t(const std::vector<cv::flann::FlannIndexType>* instance, size_t index, cv::flann::FlannIndexType* ocvrs_return) {
			cv::flann::FlannIndexType ret = (*instance)[index];
			*ocvrs_return = ret;
	}

	// std::vector<cv::flann::FlannIndexType>::set(Primitive, Enum) generated
	// ("std::vector<cv::flann::FlannIndexType>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::flann::FlannIndexType"]), _)]),
	void std_vectorLcv_flann_FlannIndexTypeG_set_size_t_const_FlannIndexType(std::vector<cv::flann::FlannIndexType>* instance, size_t index, const cv::flann::FlannIndexType val) {
			(*instance)[index] = val;
	}

	// std::vector<cv::flann::FlannIndexType>::clone() generated
	// ("std::vector<cv::flann::FlannIndexType>::clone", vec![(pred!(const, [], []), _)]),
	std::vector<cv::flann::FlannIndexType>* std_vectorLcv_flann_FlannIndexTypeG_clone_const(const std::vector<cv::flann::FlannIndexType>* instance) {
			std::vector<cv::flann::FlannIndexType> ret = std::vector<cv::flann::FlannIndexType>(*instance);
			return new std::vector<cv::flann::FlannIndexType>(ret);
	}

	// std::vector<cv::flann::FlannIndexType>::data() generated
	// ("std::vector<cv::flann::FlannIndexType>::data", vec![(pred!(const, [], []), _)]),
	const cv::flann::FlannIndexType* std_vectorLcv_flann_FlannIndexTypeG_data_const(const std::vector<cv::flann::FlannIndexType>* instance) {
			const cv::flann::FlannIndexType* ret = instance->data();
			return ret;
	}

	// std::vector<cv::flann::FlannIndexType>::dataMut() generated
	// ("std::vector<cv::flann::FlannIndexType>::dataMut", vec![(pred!(mut, [], []), _)]),
	cv::flann::FlannIndexType* std_vectorLcv_flann_FlannIndexTypeG_dataMut(std::vector<cv::flann::FlannIndexType>* instance) {
			cv::flann::FlannIndexType* ret = instance->data();
			return ret;
	}

	// cv::fromSlice(Enum, Primitive) generated
	// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const cv::flann::FlannIndexType*", "size_t"]), _)]),
	std::vector<cv::flann::FlannIndexType>* cv_fromSlice_const_const_FlannIndexTypeX_size_t(const cv::flann::FlannIndexType* data, size_t len) {
			return new std::vector<cv::flann::FlannIndexType>(data, data + len);
	}

}


