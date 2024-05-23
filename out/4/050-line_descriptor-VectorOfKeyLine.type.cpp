extern "C" {
	// std::vector<cv::line_descriptor::KeyLine>::new() generated
	// ("std::vector<cv::line_descriptor::KeyLine>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::line_descriptor::KeyLine>* std_vectorLcv_line_descriptor_KeyLineG_new_const() {
			std::vector<cv::line_descriptor::KeyLine>* ret = new std::vector<cv::line_descriptor::KeyLine>();
			return ret;
	}

	// std::vector<cv::line_descriptor::KeyLine>::delete() generated
	// ("std::vector<cv::line_descriptor::KeyLine>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_line_descriptor_KeyLineG_delete(std::vector<cv::line_descriptor::KeyLine>* instance) {
			delete instance;
	}

	// std::vector<cv::line_descriptor::KeyLine>::len() generated
	// ("std::vector<cv::line_descriptor::KeyLine>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_line_descriptor_KeyLineG_len_const(const std::vector<cv::line_descriptor::KeyLine>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::line_descriptor::KeyLine>::isEmpty() generated
	// ("std::vector<cv::line_descriptor::KeyLine>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_line_descriptor_KeyLineG_isEmpty_const(const std::vector<cv::line_descriptor::KeyLine>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::line_descriptor::KeyLine>::capacity() generated
	// ("std::vector<cv::line_descriptor::KeyLine>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_line_descriptor_KeyLineG_capacity_const(const std::vector<cv::line_descriptor::KeyLine>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::line_descriptor::KeyLine>::shrinkToFit() generated
	// ("std::vector<cv::line_descriptor::KeyLine>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_line_descriptor_KeyLineG_shrinkToFit(std::vector<cv::line_descriptor::KeyLine>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::line_descriptor::KeyLine>::reserve(Primitive) generated
	// ("std::vector<cv::line_descriptor::KeyLine>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_line_descriptor_KeyLineG_reserve_size_t(std::vector<cv::line_descriptor::KeyLine>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::line_descriptor::KeyLine>::remove(Primitive) generated
	// ("std::vector<cv::line_descriptor::KeyLine>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_line_descriptor_KeyLineG_remove_size_t(std::vector<cv::line_descriptor::KeyLine>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::line_descriptor::KeyLine>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::line_descriptor::KeyLine>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_line_descriptor_KeyLineG_swap_size_t_size_t(std::vector<cv::line_descriptor::KeyLine>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::line_descriptor::KeyLine>::clear() generated
	// ("std::vector<cv::line_descriptor::KeyLine>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_line_descriptor_KeyLineG_clear(std::vector<cv::line_descriptor::KeyLine>* instance) {
			instance->clear();
	}

	// std::vector<cv::line_descriptor::KeyLine>::push(SimpleClass) generated
	// ("std::vector<cv::line_descriptor::KeyLine>::push", vec![(pred!(mut, ["val"], ["const cv::line_descriptor::KeyLine"]), _)]),
	void std_vectorLcv_line_descriptor_KeyLineG_push_const_KeyLine(std::vector<cv::line_descriptor::KeyLine>* instance, const cv::line_descriptor::KeyLine* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::line_descriptor::KeyLine>::insert(Primitive, SimpleClass) generated
	// ("std::vector<cv::line_descriptor::KeyLine>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::line_descriptor::KeyLine"]), _)]),
	void std_vectorLcv_line_descriptor_KeyLineG_insert_size_t_const_KeyLine(std::vector<cv::line_descriptor::KeyLine>* instance, size_t index, const cv::line_descriptor::KeyLine* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::line_descriptor::KeyLine>::get(Primitive) generated
	// ("std::vector<cv::line_descriptor::KeyLine>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_line_descriptor_KeyLineG_get_const_size_t(const std::vector<cv::line_descriptor::KeyLine>* instance, size_t index, cv::line_descriptor::KeyLine* ocvrs_return) {
			cv::line_descriptor::KeyLine ret = (*instance)[index];
			*ocvrs_return = ret;
	}

	// std::vector<cv::line_descriptor::KeyLine>::set(Primitive, SimpleClass) generated
	// ("std::vector<cv::line_descriptor::KeyLine>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::line_descriptor::KeyLine"]), _)]),
	void std_vectorLcv_line_descriptor_KeyLineG_set_size_t_const_KeyLine(std::vector<cv::line_descriptor::KeyLine>* instance, size_t index, const cv::line_descriptor::KeyLine* val) {
			(*instance)[index] = *val;
	}

	// std::vector<cv::line_descriptor::KeyLine>::clone() generated
	// ("std::vector<cv::line_descriptor::KeyLine>::clone", vec![(pred!(const, [], []), _)]),
	std::vector<cv::line_descriptor::KeyLine>* std_vectorLcv_line_descriptor_KeyLineG_clone_const(const std::vector<cv::line_descriptor::KeyLine>* instance) {
			std::vector<cv::line_descriptor::KeyLine> ret = std::vector<cv::line_descriptor::KeyLine>(*instance);
			return new std::vector<cv::line_descriptor::KeyLine>(ret);
	}

	// std::vector<cv::line_descriptor::KeyLine>::data() generated
	// ("std::vector<cv::line_descriptor::KeyLine>::data", vec![(pred!(const, [], []), _)]),
	const cv::line_descriptor::KeyLine* std_vectorLcv_line_descriptor_KeyLineG_data_const(const std::vector<cv::line_descriptor::KeyLine>* instance) {
			const cv::line_descriptor::KeyLine* ret = instance->data();
			return ret;
	}

	// std::vector<cv::line_descriptor::KeyLine>::dataMut() generated
	// ("std::vector<cv::line_descriptor::KeyLine>::dataMut", vec![(pred!(mut, [], []), _)]),
	cv::line_descriptor::KeyLine* std_vectorLcv_line_descriptor_KeyLineG_dataMut(std::vector<cv::line_descriptor::KeyLine>* instance) {
			cv::line_descriptor::KeyLine* ret = instance->data();
			return ret;
	}

	// cv::fromSlice(SimpleClass, Primitive) generated
	// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const cv::line_descriptor::KeyLine*", "size_t"]), _)]),
	std::vector<cv::line_descriptor::KeyLine>* cv_fromSlice_const_const_KeyLineX_size_t(const cv::line_descriptor::KeyLine* data, size_t len) {
			return new std::vector<cv::line_descriptor::KeyLine>(data, data + len);
	}

}


