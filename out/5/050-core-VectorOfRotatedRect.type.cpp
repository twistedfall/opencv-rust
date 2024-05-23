extern "C" {
	// std::vector<cv::RotatedRect>::new() generated
	// ("std::vector<cv::RotatedRect>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::RotatedRect>* std_vectorLcv_RotatedRectG_new_const() {
			std::vector<cv::RotatedRect>* ret = new std::vector<cv::RotatedRect>();
			return ret;
	}

	// std::vector<cv::RotatedRect>::delete() generated
	// ("std::vector<cv::RotatedRect>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_RotatedRectG_delete(std::vector<cv::RotatedRect>* instance) {
			delete instance;
	}

	// std::vector<cv::RotatedRect>::len() generated
	// ("std::vector<cv::RotatedRect>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_RotatedRectG_len_const(const std::vector<cv::RotatedRect>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::RotatedRect>::isEmpty() generated
	// ("std::vector<cv::RotatedRect>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_RotatedRectG_isEmpty_const(const std::vector<cv::RotatedRect>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::RotatedRect>::capacity() generated
	// ("std::vector<cv::RotatedRect>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_RotatedRectG_capacity_const(const std::vector<cv::RotatedRect>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::RotatedRect>::shrinkToFit() generated
	// ("std::vector<cv::RotatedRect>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_RotatedRectG_shrinkToFit(std::vector<cv::RotatedRect>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::RotatedRect>::reserve(Primitive) generated
	// ("std::vector<cv::RotatedRect>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_RotatedRectG_reserve_size_t(std::vector<cv::RotatedRect>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::RotatedRect>::remove(Primitive) generated
	// ("std::vector<cv::RotatedRect>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_RotatedRectG_remove_size_t(std::vector<cv::RotatedRect>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::RotatedRect>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::RotatedRect>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_RotatedRectG_swap_size_t_size_t(std::vector<cv::RotatedRect>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::RotatedRect>::clear() generated
	// ("std::vector<cv::RotatedRect>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_RotatedRectG_clear(std::vector<cv::RotatedRect>* instance) {
			instance->clear();
	}

	// std::vector<cv::RotatedRect>::push(SimpleClass) generated
	// ("std::vector<cv::RotatedRect>::push", vec![(pred!(mut, ["val"], ["const cv::RotatedRect"]), _)]),
	void std_vectorLcv_RotatedRectG_push_const_RotatedRect(std::vector<cv::RotatedRect>* instance, const cv::RotatedRect* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::RotatedRect>::insert(Primitive, SimpleClass) generated
	// ("std::vector<cv::RotatedRect>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::RotatedRect"]), _)]),
	void std_vectorLcv_RotatedRectG_insert_size_t_const_RotatedRect(std::vector<cv::RotatedRect>* instance, size_t index, const cv::RotatedRect* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::RotatedRect>::get(Primitive) generated
	// ("std::vector<cv::RotatedRect>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_RotatedRectG_get_const_size_t(const std::vector<cv::RotatedRect>* instance, size_t index, cv::RotatedRect* ocvrs_return) {
			cv::RotatedRect ret = (*instance)[index];
			*ocvrs_return = ret;
	}

	// std::vector<cv::RotatedRect>::set(Primitive, SimpleClass) generated
	// ("std::vector<cv::RotatedRect>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::RotatedRect"]), _)]),
	void std_vectorLcv_RotatedRectG_set_size_t_const_RotatedRect(std::vector<cv::RotatedRect>* instance, size_t index, const cv::RotatedRect* val) {
			(*instance)[index] = *val;
	}

	// std::vector<cv::RotatedRect>::clone() generated
	// ("std::vector<cv::RotatedRect>::clone", vec![(pred!(const, [], []), _)]),
	std::vector<cv::RotatedRect>* std_vectorLcv_RotatedRectG_clone_const(const std::vector<cv::RotatedRect>* instance) {
			std::vector<cv::RotatedRect> ret = std::vector<cv::RotatedRect>(*instance);
			return new std::vector<cv::RotatedRect>(ret);
	}

	// std::vector<cv::RotatedRect>::data() generated
	// ("std::vector<cv::RotatedRect>::data", vec![(pred!(const, [], []), _)]),
	const cv::RotatedRect* std_vectorLcv_RotatedRectG_data_const(const std::vector<cv::RotatedRect>* instance) {
			const cv::RotatedRect* ret = instance->data();
			return ret;
	}

	// std::vector<cv::RotatedRect>::dataMut() generated
	// ("std::vector<cv::RotatedRect>::dataMut", vec![(pred!(mut, [], []), _)]),
	cv::RotatedRect* std_vectorLcv_RotatedRectG_dataMut(std::vector<cv::RotatedRect>* instance) {
			cv::RotatedRect* ret = instance->data();
			return ret;
	}

	// cv::fromSlice(SimpleClass, Primitive) generated
	// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const cv::RotatedRect*", "size_t"]), _)]),
	std::vector<cv::RotatedRect>* cv_fromSlice_const_const_RotatedRectX_size_t(const cv::RotatedRect* data, size_t len) {
			return new std::vector<cv::RotatedRect>(data, data + len);
	}

}


