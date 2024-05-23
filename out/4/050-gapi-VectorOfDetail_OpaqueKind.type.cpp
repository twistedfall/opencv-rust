extern "C" {
	// std::vector<cv::detail::OpaqueKind>::new() generated
	// ("std::vector<cv::detail::OpaqueKind>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::detail::OpaqueKind>* std_vectorLcv_detail_OpaqueKindG_new_const() {
			std::vector<cv::detail::OpaqueKind>* ret = new std::vector<cv::detail::OpaqueKind>();
			return ret;
	}

	// std::vector<cv::detail::OpaqueKind>::delete() generated
	// ("std::vector<cv::detail::OpaqueKind>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_detail_OpaqueKindG_delete(std::vector<cv::detail::OpaqueKind>* instance) {
			delete instance;
	}

	// std::vector<cv::detail::OpaqueKind>::len() generated
	// ("std::vector<cv::detail::OpaqueKind>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_detail_OpaqueKindG_len_const(const std::vector<cv::detail::OpaqueKind>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::detail::OpaqueKind>::isEmpty() generated
	// ("std::vector<cv::detail::OpaqueKind>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_detail_OpaqueKindG_isEmpty_const(const std::vector<cv::detail::OpaqueKind>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::detail::OpaqueKind>::capacity() generated
	// ("std::vector<cv::detail::OpaqueKind>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_detail_OpaqueKindG_capacity_const(const std::vector<cv::detail::OpaqueKind>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::detail::OpaqueKind>::shrinkToFit() generated
	// ("std::vector<cv::detail::OpaqueKind>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_detail_OpaqueKindG_shrinkToFit(std::vector<cv::detail::OpaqueKind>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::detail::OpaqueKind>::reserve(Primitive) generated
	// ("std::vector<cv::detail::OpaqueKind>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_detail_OpaqueKindG_reserve_size_t(std::vector<cv::detail::OpaqueKind>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::detail::OpaqueKind>::remove(Primitive) generated
	// ("std::vector<cv::detail::OpaqueKind>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_detail_OpaqueKindG_remove_size_t(std::vector<cv::detail::OpaqueKind>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::detail::OpaqueKind>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::detail::OpaqueKind>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_detail_OpaqueKindG_swap_size_t_size_t(std::vector<cv::detail::OpaqueKind>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::detail::OpaqueKind>::clear() generated
	// ("std::vector<cv::detail::OpaqueKind>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_detail_OpaqueKindG_clear(std::vector<cv::detail::OpaqueKind>* instance) {
			instance->clear();
	}

	// std::vector<cv::detail::OpaqueKind>::push(Enum) generated
	// ("std::vector<cv::detail::OpaqueKind>::push", vec![(pred!(mut, ["val"], ["const cv::detail::OpaqueKind"]), _)]),
	void std_vectorLcv_detail_OpaqueKindG_push_const_OpaqueKind(std::vector<cv::detail::OpaqueKind>* instance, const cv::detail::OpaqueKind val) {
			instance->push_back(val);
	}

	// std::vector<cv::detail::OpaqueKind>::insert(Primitive, Enum) generated
	// ("std::vector<cv::detail::OpaqueKind>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::detail::OpaqueKind"]), _)]),
	void std_vectorLcv_detail_OpaqueKindG_insert_size_t_const_OpaqueKind(std::vector<cv::detail::OpaqueKind>* instance, size_t index, const cv::detail::OpaqueKind val) {
			instance->insert(instance->begin() + index, val);
	}

	// std::vector<cv::detail::OpaqueKind>::get(Primitive) generated
	// ("std::vector<cv::detail::OpaqueKind>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_detail_OpaqueKindG_get_const_size_t(const std::vector<cv::detail::OpaqueKind>* instance, size_t index, cv::detail::OpaqueKind* ocvrs_return) {
			cv::detail::OpaqueKind ret = (*instance)[index];
			*ocvrs_return = ret;
	}

	// std::vector<cv::detail::OpaqueKind>::set(Primitive, Enum) generated
	// ("std::vector<cv::detail::OpaqueKind>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::detail::OpaqueKind"]), _)]),
	void std_vectorLcv_detail_OpaqueKindG_set_size_t_const_OpaqueKind(std::vector<cv::detail::OpaqueKind>* instance, size_t index, const cv::detail::OpaqueKind val) {
			(*instance)[index] = val;
	}

	// std::vector<cv::detail::OpaqueKind>::clone() generated
	// ("std::vector<cv::detail::OpaqueKind>::clone", vec![(pred!(const, [], []), _)]),
	std::vector<cv::detail::OpaqueKind>* std_vectorLcv_detail_OpaqueKindG_clone_const(const std::vector<cv::detail::OpaqueKind>* instance) {
			std::vector<cv::detail::OpaqueKind> ret = std::vector<cv::detail::OpaqueKind>(*instance);
			return new std::vector<cv::detail::OpaqueKind>(ret);
	}

	// std::vector<cv::detail::OpaqueKind>::data() generated
	// ("std::vector<cv::detail::OpaqueKind>::data", vec![(pred!(const, [], []), _)]),
	const cv::detail::OpaqueKind* std_vectorLcv_detail_OpaqueKindG_data_const(const std::vector<cv::detail::OpaqueKind>* instance) {
			const cv::detail::OpaqueKind* ret = instance->data();
			return ret;
	}

	// std::vector<cv::detail::OpaqueKind>::dataMut() generated
	// ("std::vector<cv::detail::OpaqueKind>::dataMut", vec![(pred!(mut, [], []), _)]),
	cv::detail::OpaqueKind* std_vectorLcv_detail_OpaqueKindG_dataMut(std::vector<cv::detail::OpaqueKind>* instance) {
			cv::detail::OpaqueKind* ret = instance->data();
			return ret;
	}

	// cv::fromSlice(Enum, Primitive) generated
	// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const cv::detail::OpaqueKind*", "size_t"]), _)]),
	std::vector<cv::detail::OpaqueKind>* cv_fromSlice_const_const_OpaqueKindX_size_t(const cv::detail::OpaqueKind* data, size_t len) {
			return new std::vector<cv::detail::OpaqueKind>(data, data + len);
	}

}


