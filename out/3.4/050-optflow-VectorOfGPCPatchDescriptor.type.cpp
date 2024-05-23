extern "C" {
	// std::vector<cv::optflow::GPCPatchDescriptor>::new() generated
	// ("std::vector<cv::optflow::GPCPatchDescriptor>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::optflow::GPCPatchDescriptor>* std_vectorLcv_optflow_GPCPatchDescriptorG_new_const() {
			std::vector<cv::optflow::GPCPatchDescriptor>* ret = new std::vector<cv::optflow::GPCPatchDescriptor>();
			return ret;
	}

	// std::vector<cv::optflow::GPCPatchDescriptor>::delete() generated
	// ("std::vector<cv::optflow::GPCPatchDescriptor>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_optflow_GPCPatchDescriptorG_delete(std::vector<cv::optflow::GPCPatchDescriptor>* instance) {
			delete instance;
	}

	// std::vector<cv::optflow::GPCPatchDescriptor>::len() generated
	// ("std::vector<cv::optflow::GPCPatchDescriptor>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_optflow_GPCPatchDescriptorG_len_const(const std::vector<cv::optflow::GPCPatchDescriptor>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::optflow::GPCPatchDescriptor>::isEmpty() generated
	// ("std::vector<cv::optflow::GPCPatchDescriptor>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_optflow_GPCPatchDescriptorG_isEmpty_const(const std::vector<cv::optflow::GPCPatchDescriptor>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::optflow::GPCPatchDescriptor>::capacity() generated
	// ("std::vector<cv::optflow::GPCPatchDescriptor>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_optflow_GPCPatchDescriptorG_capacity_const(const std::vector<cv::optflow::GPCPatchDescriptor>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::optflow::GPCPatchDescriptor>::shrinkToFit() generated
	// ("std::vector<cv::optflow::GPCPatchDescriptor>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_optflow_GPCPatchDescriptorG_shrinkToFit(std::vector<cv::optflow::GPCPatchDescriptor>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::optflow::GPCPatchDescriptor>::reserve(Primitive) generated
	// ("std::vector<cv::optflow::GPCPatchDescriptor>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_optflow_GPCPatchDescriptorG_reserve_size_t(std::vector<cv::optflow::GPCPatchDescriptor>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::optflow::GPCPatchDescriptor>::remove(Primitive) generated
	// ("std::vector<cv::optflow::GPCPatchDescriptor>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_optflow_GPCPatchDescriptorG_remove_size_t(std::vector<cv::optflow::GPCPatchDescriptor>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::optflow::GPCPatchDescriptor>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::optflow::GPCPatchDescriptor>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_optflow_GPCPatchDescriptorG_swap_size_t_size_t(std::vector<cv::optflow::GPCPatchDescriptor>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::optflow::GPCPatchDescriptor>::clear() generated
	// ("std::vector<cv::optflow::GPCPatchDescriptor>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_optflow_GPCPatchDescriptorG_clear(std::vector<cv::optflow::GPCPatchDescriptor>* instance) {
			instance->clear();
	}

	// std::vector<cv::optflow::GPCPatchDescriptor>::push(TraitClass) generated
	// ("std::vector<cv::optflow::GPCPatchDescriptor>::push", vec![(pred!(mut, ["val"], ["const cv::optflow::GPCPatchDescriptor"]), _)]),
	void std_vectorLcv_optflow_GPCPatchDescriptorG_push_const_GPCPatchDescriptor(std::vector<cv::optflow::GPCPatchDescriptor>* instance, const cv::optflow::GPCPatchDescriptor* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::optflow::GPCPatchDescriptor>::insert(Primitive, TraitClass) generated
	// ("std::vector<cv::optflow::GPCPatchDescriptor>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::optflow::GPCPatchDescriptor"]), _)]),
	void std_vectorLcv_optflow_GPCPatchDescriptorG_insert_size_t_const_GPCPatchDescriptor(std::vector<cv::optflow::GPCPatchDescriptor>* instance, size_t index, const cv::optflow::GPCPatchDescriptor* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::optflow::GPCPatchDescriptor>::get(Primitive) generated
	// ("std::vector<cv::optflow::GPCPatchDescriptor>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_optflow_GPCPatchDescriptorG_get_const_size_t(const std::vector<cv::optflow::GPCPatchDescriptor>* instance, size_t index, cv::optflow::GPCPatchDescriptor** ocvrs_return) {
			cv::optflow::GPCPatchDescriptor ret = (*instance)[index];
			*ocvrs_return = new cv::optflow::GPCPatchDescriptor(ret);
	}

	// std::vector<cv::optflow::GPCPatchDescriptor>::set(Primitive, TraitClass) generated
	// ("std::vector<cv::optflow::GPCPatchDescriptor>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::optflow::GPCPatchDescriptor"]), _)]),
	void std_vectorLcv_optflow_GPCPatchDescriptorG_set_size_t_const_GPCPatchDescriptor(std::vector<cv::optflow::GPCPatchDescriptor>* instance, size_t index, const cv::optflow::GPCPatchDescriptor* val) {
			(*instance)[index] = *val;
	}

}


