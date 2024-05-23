extern "C" {
	// std::vector<cv::GTypeInfo>::new() generated
	// ("std::vector<cv::GTypeInfo>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::GTypeInfo>* std_vectorLcv_GTypeInfoG_new_const() {
			std::vector<cv::GTypeInfo>* ret = new std::vector<cv::GTypeInfo>();
			return ret;
	}

	// std::vector<cv::GTypeInfo>::delete() generated
	// ("std::vector<cv::GTypeInfo>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_GTypeInfoG_delete(std::vector<cv::GTypeInfo>* instance) {
			delete instance;
	}

	// std::vector<cv::GTypeInfo>::len() generated
	// ("std::vector<cv::GTypeInfo>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_GTypeInfoG_len_const(const std::vector<cv::GTypeInfo>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::GTypeInfo>::isEmpty() generated
	// ("std::vector<cv::GTypeInfo>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_GTypeInfoG_isEmpty_const(const std::vector<cv::GTypeInfo>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::GTypeInfo>::capacity() generated
	// ("std::vector<cv::GTypeInfo>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_GTypeInfoG_capacity_const(const std::vector<cv::GTypeInfo>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::GTypeInfo>::shrinkToFit() generated
	// ("std::vector<cv::GTypeInfo>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_GTypeInfoG_shrinkToFit(std::vector<cv::GTypeInfo>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::GTypeInfo>::reserve(Primitive) generated
	// ("std::vector<cv::GTypeInfo>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_GTypeInfoG_reserve_size_t(std::vector<cv::GTypeInfo>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::GTypeInfo>::remove(Primitive) generated
	// ("std::vector<cv::GTypeInfo>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_GTypeInfoG_remove_size_t(std::vector<cv::GTypeInfo>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::GTypeInfo>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::GTypeInfo>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_GTypeInfoG_swap_size_t_size_t(std::vector<cv::GTypeInfo>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::GTypeInfo>::clear() generated
	// ("std::vector<cv::GTypeInfo>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_GTypeInfoG_clear(std::vector<cv::GTypeInfo>* instance) {
			instance->clear();
	}

	// std::vector<cv::GTypeInfo>::push(TraitClass) generated
	// ("std::vector<cv::GTypeInfo>::push", vec![(pred!(mut, ["val"], ["const cv::GTypeInfo"]), _)]),
	void std_vectorLcv_GTypeInfoG_push_const_GTypeInfo(std::vector<cv::GTypeInfo>* instance, const cv::GTypeInfo* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::GTypeInfo>::insert(Primitive, TraitClass) generated
	// ("std::vector<cv::GTypeInfo>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::GTypeInfo"]), _)]),
	void std_vectorLcv_GTypeInfoG_insert_size_t_const_GTypeInfo(std::vector<cv::GTypeInfo>* instance, size_t index, const cv::GTypeInfo* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::GTypeInfo>::get(Primitive) generated
	// ("std::vector<cv::GTypeInfo>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_GTypeInfoG_get_const_size_t(const std::vector<cv::GTypeInfo>* instance, size_t index, cv::GTypeInfo** ocvrs_return) {
			cv::GTypeInfo ret = (*instance)[index];
			*ocvrs_return = new cv::GTypeInfo(ret);
	}

	// std::vector<cv::GTypeInfo>::set(Primitive, TraitClass) generated
	// ("std::vector<cv::GTypeInfo>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::GTypeInfo"]), _)]),
	void std_vectorLcv_GTypeInfoG_set_size_t_const_GTypeInfo(std::vector<cv::GTypeInfo>* instance, size_t index, const cv::GTypeInfo* val) {
			(*instance)[index] = *val;
	}

}


