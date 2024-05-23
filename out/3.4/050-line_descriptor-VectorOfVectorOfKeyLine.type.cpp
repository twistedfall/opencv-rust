extern "C" {
	// std::vector<std::vector<cv::line_descriptor::KeyLine>>::new() generated
	// ("std::vector<std::vector<cv::line_descriptor::KeyLine>>::new", vec![(pred!(const, [], []), _)]),
	std::vector<std::vector<cv::line_descriptor::KeyLine>>* std_vectorLstd_vectorLcv_line_descriptor_KeyLineGG_new_const() {
			std::vector<std::vector<cv::line_descriptor::KeyLine>>* ret = new std::vector<std::vector<cv::line_descriptor::KeyLine>>();
			return ret;
	}

	// std::vector<std::vector<cv::line_descriptor::KeyLine>>::delete() generated
	// ("std::vector<std::vector<cv::line_descriptor::KeyLine>>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_line_descriptor_KeyLineGG_delete(std::vector<std::vector<cv::line_descriptor::KeyLine>>* instance) {
			delete instance;
	}

	// std::vector<std::vector<cv::line_descriptor::KeyLine>>::len() generated
	// ("std::vector<std::vector<cv::line_descriptor::KeyLine>>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_vectorLcv_line_descriptor_KeyLineGG_len_const(const std::vector<std::vector<cv::line_descriptor::KeyLine>>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<std::vector<cv::line_descriptor::KeyLine>>::isEmpty() generated
	// ("std::vector<std::vector<cv::line_descriptor::KeyLine>>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLstd_vectorLcv_line_descriptor_KeyLineGG_isEmpty_const(const std::vector<std::vector<cv::line_descriptor::KeyLine>>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<std::vector<cv::line_descriptor::KeyLine>>::capacity() generated
	// ("std::vector<std::vector<cv::line_descriptor::KeyLine>>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_vectorLcv_line_descriptor_KeyLineGG_capacity_const(const std::vector<std::vector<cv::line_descriptor::KeyLine>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<std::vector<cv::line_descriptor::KeyLine>>::shrinkToFit() generated
	// ("std::vector<std::vector<cv::line_descriptor::KeyLine>>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_line_descriptor_KeyLineGG_shrinkToFit(std::vector<std::vector<cv::line_descriptor::KeyLine>>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<std::vector<cv::line_descriptor::KeyLine>>::reserve(Primitive) generated
	// ("std::vector<std::vector<cv::line_descriptor::KeyLine>>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLcv_line_descriptor_KeyLineGG_reserve_size_t(std::vector<std::vector<cv::line_descriptor::KeyLine>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<std::vector<cv::line_descriptor::KeyLine>>::remove(Primitive) generated
	// ("std::vector<std::vector<cv::line_descriptor::KeyLine>>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLcv_line_descriptor_KeyLineGG_remove_size_t(std::vector<std::vector<cv::line_descriptor::KeyLine>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<std::vector<cv::line_descriptor::KeyLine>>::swap(Primitive, Primitive) generated
	// ("std::vector<std::vector<cv::line_descriptor::KeyLine>>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLstd_vectorLcv_line_descriptor_KeyLineGG_swap_size_t_size_t(std::vector<std::vector<cv::line_descriptor::KeyLine>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<std::vector<cv::line_descriptor::KeyLine>>::clear() generated
	// ("std::vector<std::vector<cv::line_descriptor::KeyLine>>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_line_descriptor_KeyLineGG_clear(std::vector<std::vector<cv::line_descriptor::KeyLine>>* instance) {
			instance->clear();
	}

	// std::vector<std::vector<cv::line_descriptor::KeyLine>>::push(CppPassByVoidPtr) generated
	// ("std::vector<std::vector<cv::line_descriptor::KeyLine>>::push", vec![(pred!(mut, ["val"], ["const std::vector<cv::line_descriptor::KeyLine>"]), _)]),
	void std_vectorLstd_vectorLcv_line_descriptor_KeyLineGG_push_const_vectorLKeyLineG(std::vector<std::vector<cv::line_descriptor::KeyLine>>* instance, const std::vector<cv::line_descriptor::KeyLine>* val) {
			instance->push_back(*val);
	}

	// std::vector<std::vector<cv::line_descriptor::KeyLine>>::insert(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::vector<cv::line_descriptor::KeyLine>>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::vector<cv::line_descriptor::KeyLine>"]), _)]),
	void std_vectorLstd_vectorLcv_line_descriptor_KeyLineGG_insert_size_t_const_vectorLKeyLineG(std::vector<std::vector<cv::line_descriptor::KeyLine>>* instance, size_t index, const std::vector<cv::line_descriptor::KeyLine>* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<std::vector<cv::line_descriptor::KeyLine>>::get(Primitive) generated
	// ("std::vector<std::vector<cv::line_descriptor::KeyLine>>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLcv_line_descriptor_KeyLineGG_get_const_size_t(const std::vector<std::vector<cv::line_descriptor::KeyLine>>* instance, size_t index, std::vector<cv::line_descriptor::KeyLine>** ocvrs_return) {
			std::vector<cv::line_descriptor::KeyLine> ret = (*instance)[index];
			*ocvrs_return = new std::vector<cv::line_descriptor::KeyLine>(ret);
	}

	// std::vector<std::vector<cv::line_descriptor::KeyLine>>::set(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::vector<cv::line_descriptor::KeyLine>>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::vector<cv::line_descriptor::KeyLine>"]), _)]),
	void std_vectorLstd_vectorLcv_line_descriptor_KeyLineGG_set_size_t_const_vectorLKeyLineG(std::vector<std::vector<cv::line_descriptor::KeyLine>>* instance, size_t index, const std::vector<cv::line_descriptor::KeyLine>* val) {
			(*instance)[index] = *val;
	}

}


