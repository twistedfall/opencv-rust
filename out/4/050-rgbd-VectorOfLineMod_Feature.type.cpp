extern "C" {
	// std::vector<cv::linemod::Feature>::new() generated
	// ("std::vector<cv::linemod::Feature>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::linemod::Feature>* std_vectorLcv_linemod_FeatureG_new_const() {
			std::vector<cv::linemod::Feature>* ret = new std::vector<cv::linemod::Feature>();
			return ret;
	}

	// std::vector<cv::linemod::Feature>::delete() generated
	// ("std::vector<cv::linemod::Feature>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_linemod_FeatureG_delete(std::vector<cv::linemod::Feature>* instance) {
			delete instance;
	}

	// std::vector<cv::linemod::Feature>::len() generated
	// ("std::vector<cv::linemod::Feature>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_linemod_FeatureG_len_const(const std::vector<cv::linemod::Feature>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::linemod::Feature>::isEmpty() generated
	// ("std::vector<cv::linemod::Feature>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_linemod_FeatureG_isEmpty_const(const std::vector<cv::linemod::Feature>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::linemod::Feature>::capacity() generated
	// ("std::vector<cv::linemod::Feature>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_linemod_FeatureG_capacity_const(const std::vector<cv::linemod::Feature>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::linemod::Feature>::shrinkToFit() generated
	// ("std::vector<cv::linemod::Feature>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_linemod_FeatureG_shrinkToFit(std::vector<cv::linemod::Feature>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::linemod::Feature>::reserve(Primitive) generated
	// ("std::vector<cv::linemod::Feature>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_linemod_FeatureG_reserve_size_t(std::vector<cv::linemod::Feature>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::linemod::Feature>::remove(Primitive) generated
	// ("std::vector<cv::linemod::Feature>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_linemod_FeatureG_remove_size_t(std::vector<cv::linemod::Feature>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::linemod::Feature>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::linemod::Feature>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_linemod_FeatureG_swap_size_t_size_t(std::vector<cv::linemod::Feature>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::linemod::Feature>::clear() generated
	// ("std::vector<cv::linemod::Feature>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_linemod_FeatureG_clear(std::vector<cv::linemod::Feature>* instance) {
			instance->clear();
	}

	// std::vector<cv::linemod::Feature>::push(SimpleClass) generated
	// ("std::vector<cv::linemod::Feature>::push", vec![(pred!(mut, ["val"], ["const cv::linemod::Feature"]), _)]),
	void std_vectorLcv_linemod_FeatureG_push_const_Feature(std::vector<cv::linemod::Feature>* instance, const cv::linemod::Feature* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::linemod::Feature>::insert(Primitive, SimpleClass) generated
	// ("std::vector<cv::linemod::Feature>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::linemod::Feature"]), _)]),
	void std_vectorLcv_linemod_FeatureG_insert_size_t_const_Feature(std::vector<cv::linemod::Feature>* instance, size_t index, const cv::linemod::Feature* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::linemod::Feature>::get(Primitive) generated
	// ("std::vector<cv::linemod::Feature>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_linemod_FeatureG_get_const_size_t(const std::vector<cv::linemod::Feature>* instance, size_t index, cv::linemod::Feature* ocvrs_return) {
			cv::linemod::Feature ret = (*instance)[index];
			*ocvrs_return = ret;
	}

	// std::vector<cv::linemod::Feature>::set(Primitive, SimpleClass) generated
	// ("std::vector<cv::linemod::Feature>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::linemod::Feature"]), _)]),
	void std_vectorLcv_linemod_FeatureG_set_size_t_const_Feature(std::vector<cv::linemod::Feature>* instance, size_t index, const cv::linemod::Feature* val) {
			(*instance)[index] = *val;
	}

	// std::vector<cv::linemod::Feature>::clone() generated
	// ("std::vector<cv::linemod::Feature>::clone", vec![(pred!(const, [], []), _)]),
	std::vector<cv::linemod::Feature>* std_vectorLcv_linemod_FeatureG_clone_const(const std::vector<cv::linemod::Feature>* instance) {
			std::vector<cv::linemod::Feature> ret = std::vector<cv::linemod::Feature>(*instance);
			return new std::vector<cv::linemod::Feature>(ret);
	}

	// std::vector<cv::linemod::Feature>::data() generated
	// ("std::vector<cv::linemod::Feature>::data", vec![(pred!(const, [], []), _)]),
	const cv::linemod::Feature* std_vectorLcv_linemod_FeatureG_data_const(const std::vector<cv::linemod::Feature>* instance) {
			const cv::linemod::Feature* ret = instance->data();
			return ret;
	}

	// std::vector<cv::linemod::Feature>::dataMut() generated
	// ("std::vector<cv::linemod::Feature>::dataMut", vec![(pred!(mut, [], []), _)]),
	cv::linemod::Feature* std_vectorLcv_linemod_FeatureG_dataMut(std::vector<cv::linemod::Feature>* instance) {
			cv::linemod::Feature* ret = instance->data();
			return ret;
	}

	// cv::fromSlice(SimpleClass, Primitive) generated
	// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const cv::linemod::Feature*", "size_t"]), _)]),
	std::vector<cv::linemod::Feature>* cv_fromSlice_const_const_FeatureX_size_t(const cv::linemod::Feature* data, size_t len) {
			return new std::vector<cv::linemod::Feature>(data, data + len);
	}

}


