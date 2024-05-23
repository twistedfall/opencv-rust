extern "C" {
	// std::vector<cv::stereo::MatchQuasiDense>::new() generated
	// ("std::vector<cv::stereo::MatchQuasiDense>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::stereo::MatchQuasiDense>* std_vectorLcv_stereo_MatchQuasiDenseG_new_const() {
			std::vector<cv::stereo::MatchQuasiDense>* ret = new std::vector<cv::stereo::MatchQuasiDense>();
			return ret;
	}

	// std::vector<cv::stereo::MatchQuasiDense>::delete() generated
	// ("std::vector<cv::stereo::MatchQuasiDense>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_stereo_MatchQuasiDenseG_delete(std::vector<cv::stereo::MatchQuasiDense>* instance) {
			delete instance;
	}

	// std::vector<cv::stereo::MatchQuasiDense>::len() generated
	// ("std::vector<cv::stereo::MatchQuasiDense>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_stereo_MatchQuasiDenseG_len_const(const std::vector<cv::stereo::MatchQuasiDense>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::stereo::MatchQuasiDense>::isEmpty() generated
	// ("std::vector<cv::stereo::MatchQuasiDense>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_stereo_MatchQuasiDenseG_isEmpty_const(const std::vector<cv::stereo::MatchQuasiDense>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::stereo::MatchQuasiDense>::capacity() generated
	// ("std::vector<cv::stereo::MatchQuasiDense>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_stereo_MatchQuasiDenseG_capacity_const(const std::vector<cv::stereo::MatchQuasiDense>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::stereo::MatchQuasiDense>::shrinkToFit() generated
	// ("std::vector<cv::stereo::MatchQuasiDense>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_stereo_MatchQuasiDenseG_shrinkToFit(std::vector<cv::stereo::MatchQuasiDense>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::stereo::MatchQuasiDense>::reserve(Primitive) generated
	// ("std::vector<cv::stereo::MatchQuasiDense>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_stereo_MatchQuasiDenseG_reserve_size_t(std::vector<cv::stereo::MatchQuasiDense>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::stereo::MatchQuasiDense>::remove(Primitive) generated
	// ("std::vector<cv::stereo::MatchQuasiDense>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_stereo_MatchQuasiDenseG_remove_size_t(std::vector<cv::stereo::MatchQuasiDense>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::stereo::MatchQuasiDense>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::stereo::MatchQuasiDense>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_stereo_MatchQuasiDenseG_swap_size_t_size_t(std::vector<cv::stereo::MatchQuasiDense>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::stereo::MatchQuasiDense>::clear() generated
	// ("std::vector<cv::stereo::MatchQuasiDense>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_stereo_MatchQuasiDenseG_clear(std::vector<cv::stereo::MatchQuasiDense>* instance) {
			instance->clear();
	}

	// std::vector<cv::stereo::MatchQuasiDense>::push(SimpleClass) generated
	// ("std::vector<cv::stereo::MatchQuasiDense>::push", vec![(pred!(mut, ["val"], ["const cv::stereo::MatchQuasiDense"]), _)]),
	void std_vectorLcv_stereo_MatchQuasiDenseG_push_const_MatchQuasiDense(std::vector<cv::stereo::MatchQuasiDense>* instance, const cv::stereo::MatchQuasiDense* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::stereo::MatchQuasiDense>::insert(Primitive, SimpleClass) generated
	// ("std::vector<cv::stereo::MatchQuasiDense>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::stereo::MatchQuasiDense"]), _)]),
	void std_vectorLcv_stereo_MatchQuasiDenseG_insert_size_t_const_MatchQuasiDense(std::vector<cv::stereo::MatchQuasiDense>* instance, size_t index, const cv::stereo::MatchQuasiDense* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::stereo::MatchQuasiDense>::get(Primitive) generated
	// ("std::vector<cv::stereo::MatchQuasiDense>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_stereo_MatchQuasiDenseG_get_const_size_t(const std::vector<cv::stereo::MatchQuasiDense>* instance, size_t index, cv::stereo::MatchQuasiDense* ocvrs_return) {
			cv::stereo::MatchQuasiDense ret = (*instance)[index];
			*ocvrs_return = ret;
	}

	// std::vector<cv::stereo::MatchQuasiDense>::set(Primitive, SimpleClass) generated
	// ("std::vector<cv::stereo::MatchQuasiDense>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::stereo::MatchQuasiDense"]), _)]),
	void std_vectorLcv_stereo_MatchQuasiDenseG_set_size_t_const_MatchQuasiDense(std::vector<cv::stereo::MatchQuasiDense>* instance, size_t index, const cv::stereo::MatchQuasiDense* val) {
			(*instance)[index] = *val;
	}

	// std::vector<cv::stereo::MatchQuasiDense>::clone() generated
	// ("std::vector<cv::stereo::MatchQuasiDense>::clone", vec![(pred!(const, [], []), _)]),
	std::vector<cv::stereo::MatchQuasiDense>* std_vectorLcv_stereo_MatchQuasiDenseG_clone_const(const std::vector<cv::stereo::MatchQuasiDense>* instance) {
			std::vector<cv::stereo::MatchQuasiDense> ret = std::vector<cv::stereo::MatchQuasiDense>(*instance);
			return new std::vector<cv::stereo::MatchQuasiDense>(ret);
	}

	// std::vector<cv::stereo::MatchQuasiDense>::data() generated
	// ("std::vector<cv::stereo::MatchQuasiDense>::data", vec![(pred!(const, [], []), _)]),
	const cv::stereo::MatchQuasiDense* std_vectorLcv_stereo_MatchQuasiDenseG_data_const(const std::vector<cv::stereo::MatchQuasiDense>* instance) {
			const cv::stereo::MatchQuasiDense* ret = instance->data();
			return ret;
	}

	// std::vector<cv::stereo::MatchQuasiDense>::dataMut() generated
	// ("std::vector<cv::stereo::MatchQuasiDense>::dataMut", vec![(pred!(mut, [], []), _)]),
	cv::stereo::MatchQuasiDense* std_vectorLcv_stereo_MatchQuasiDenseG_dataMut(std::vector<cv::stereo::MatchQuasiDense>* instance) {
			cv::stereo::MatchQuasiDense* ret = instance->data();
			return ret;
	}

	// cv::fromSlice(SimpleClass, Primitive) generated
	// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const cv::stereo::MatchQuasiDense*", "size_t"]), _)]),
	std::vector<cv::stereo::MatchQuasiDense>* cv_fromSlice_const_const_MatchQuasiDenseX_size_t(const cv::stereo::MatchQuasiDense* data, size_t len) {
			return new std::vector<cv::stereo::MatchQuasiDense>(data, data + len);
	}

}


