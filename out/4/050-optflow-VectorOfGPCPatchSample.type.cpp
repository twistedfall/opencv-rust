extern "C" {
	// std::vector<cv::optflow::GPCPatchSample>::new() generated
	// ("std::vector<cv::optflow::GPCPatchSample>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::optflow::GPCPatchSample>* std_vectorLcv_optflow_GPCPatchSampleG_new_const() {
			std::vector<cv::optflow::GPCPatchSample>* ret = new std::vector<cv::optflow::GPCPatchSample>();
			return ret;
	}

	// std::vector<cv::optflow::GPCPatchSample>::delete() generated
	// ("std::vector<cv::optflow::GPCPatchSample>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_optflow_GPCPatchSampleG_delete(std::vector<cv::optflow::GPCPatchSample>* instance) {
			delete instance;
	}

	// std::vector<cv::optflow::GPCPatchSample>::len() generated
	// ("std::vector<cv::optflow::GPCPatchSample>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_optflow_GPCPatchSampleG_len_const(const std::vector<cv::optflow::GPCPatchSample>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::optflow::GPCPatchSample>::isEmpty() generated
	// ("std::vector<cv::optflow::GPCPatchSample>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_optflow_GPCPatchSampleG_isEmpty_const(const std::vector<cv::optflow::GPCPatchSample>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::optflow::GPCPatchSample>::capacity() generated
	// ("std::vector<cv::optflow::GPCPatchSample>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_optflow_GPCPatchSampleG_capacity_const(const std::vector<cv::optflow::GPCPatchSample>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::optflow::GPCPatchSample>::shrinkToFit() generated
	// ("std::vector<cv::optflow::GPCPatchSample>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_optflow_GPCPatchSampleG_shrinkToFit(std::vector<cv::optflow::GPCPatchSample>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::optflow::GPCPatchSample>::reserve(Primitive) generated
	// ("std::vector<cv::optflow::GPCPatchSample>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_optflow_GPCPatchSampleG_reserve_size_t(std::vector<cv::optflow::GPCPatchSample>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::optflow::GPCPatchSample>::remove(Primitive) generated
	// ("std::vector<cv::optflow::GPCPatchSample>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_optflow_GPCPatchSampleG_remove_size_t(std::vector<cv::optflow::GPCPatchSample>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::optflow::GPCPatchSample>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::optflow::GPCPatchSample>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_optflow_GPCPatchSampleG_swap_size_t_size_t(std::vector<cv::optflow::GPCPatchSample>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::optflow::GPCPatchSample>::clear() generated
	// ("std::vector<cv::optflow::GPCPatchSample>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_optflow_GPCPatchSampleG_clear(std::vector<cv::optflow::GPCPatchSample>* instance) {
			instance->clear();
	}

	// std::vector<cv::optflow::GPCPatchSample>::push(TraitClass) generated
	// ("std::vector<cv::optflow::GPCPatchSample>::push", vec![(pred!(mut, ["val"], ["const cv::optflow::GPCPatchSample"]), _)]),
	void std_vectorLcv_optflow_GPCPatchSampleG_push_const_GPCPatchSample(std::vector<cv::optflow::GPCPatchSample>* instance, const cv::optflow::GPCPatchSample* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::optflow::GPCPatchSample>::insert(Primitive, TraitClass) generated
	// ("std::vector<cv::optflow::GPCPatchSample>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::optflow::GPCPatchSample"]), _)]),
	void std_vectorLcv_optflow_GPCPatchSampleG_insert_size_t_const_GPCPatchSample(std::vector<cv::optflow::GPCPatchSample>* instance, size_t index, const cv::optflow::GPCPatchSample* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::optflow::GPCPatchSample>::get(Primitive) generated
	// ("std::vector<cv::optflow::GPCPatchSample>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_optflow_GPCPatchSampleG_get_const_size_t(const std::vector<cv::optflow::GPCPatchSample>* instance, size_t index, cv::optflow::GPCPatchSample** ocvrs_return) {
			cv::optflow::GPCPatchSample ret = (*instance)[index];
			*ocvrs_return = new cv::optflow::GPCPatchSample(ret);
	}

	// std::vector<cv::optflow::GPCPatchSample>::set(Primitive, TraitClass) generated
	// ("std::vector<cv::optflow::GPCPatchSample>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::optflow::GPCPatchSample"]), _)]),
	void std_vectorLcv_optflow_GPCPatchSampleG_set_size_t_const_GPCPatchSample(std::vector<cv::optflow::GPCPatchSample>* instance, size_t index, const cv::optflow::GPCPatchSample* val) {
			(*instance)[index] = *val;
	}

}


