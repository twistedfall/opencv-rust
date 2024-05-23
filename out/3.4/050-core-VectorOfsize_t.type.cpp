extern "C" {
	// std::vector<size_t>::new() generated
	// ("std::vector<size_t>::new", vec![(pred!(const, [], []), _)]),
	std::vector<size_t>* std_vectorLsize_tG_new_const() {
			std::vector<size_t>* ret = new std::vector<size_t>();
			return ret;
	}

	// std::vector<size_t>::delete() generated
	// ("std::vector<size_t>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLsize_tG_delete(std::vector<size_t>* instance) {
			delete instance;
	}

	// std::vector<size_t>::len() generated
	// ("std::vector<size_t>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLsize_tG_len_const(const std::vector<size_t>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<size_t>::isEmpty() generated
	// ("std::vector<size_t>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLsize_tG_isEmpty_const(const std::vector<size_t>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<size_t>::capacity() generated
	// ("std::vector<size_t>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLsize_tG_capacity_const(const std::vector<size_t>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<size_t>::shrinkToFit() generated
	// ("std::vector<size_t>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLsize_tG_shrinkToFit(std::vector<size_t>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<size_t>::reserve(Primitive) generated
	// ("std::vector<size_t>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLsize_tG_reserve_size_t(std::vector<size_t>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<size_t>::remove(Primitive) generated
	// ("std::vector<size_t>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLsize_tG_remove_size_t(std::vector<size_t>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<size_t>::swap(Primitive, Primitive) generated
	// ("std::vector<size_t>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLsize_tG_swap_size_t_size_t(std::vector<size_t>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<size_t>::clear() generated
	// ("std::vector<size_t>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLsize_tG_clear(std::vector<size_t>* instance) {
			instance->clear();
	}

	// std::vector<size_t>::push(Primitive) generated
	// ("std::vector<size_t>::push", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
	void std_vectorLsize_tG_push_const_size_t(std::vector<size_t>* instance, const size_t val) {
			instance->push_back(val);
	}

	// std::vector<size_t>::insert(Primitive, Primitive) generated
	// ("std::vector<size_t>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const size_t"]), _)]),
	void std_vectorLsize_tG_insert_size_t_const_size_t(std::vector<size_t>* instance, size_t index, const size_t val) {
			instance->insert(instance->begin() + index, val);
	}

	// std::vector<size_t>::get(Primitive) generated
	// ("std::vector<size_t>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLsize_tG_get_const_size_t(const std::vector<size_t>* instance, size_t index, size_t* ocvrs_return) {
			size_t ret = (*instance)[index];
			*ocvrs_return = ret;
	}

	// std::vector<size_t>::set(Primitive, Primitive) generated
	// ("std::vector<size_t>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const size_t"]), _)]),
	void std_vectorLsize_tG_set_size_t_const_size_t(std::vector<size_t>* instance, size_t index, const size_t val) {
			(*instance)[index] = val;
	}

	// std::vector<size_t>::clone() generated
	// ("std::vector<size_t>::clone", vec![(pred!(const, [], []), _)]),
	std::vector<size_t>* std_vectorLsize_tG_clone_const(const std::vector<size_t>* instance) {
			std::vector<size_t> ret = std::vector<size_t>(*instance);
			return new std::vector<size_t>(ret);
	}

	// std::vector<size_t>::data() generated
	// ("std::vector<size_t>::data", vec![(pred!(const, [], []), _)]),
	const size_t* std_vectorLsize_tG_data_const(const std::vector<size_t>* instance) {
			const size_t* ret = instance->data();
			return ret;
	}

	// std::vector<size_t>::dataMut() generated
	// ("std::vector<size_t>::dataMut", vec![(pred!(mut, [], []), _)]),
	size_t* std_vectorLsize_tG_dataMut(std::vector<size_t>* instance) {
			size_t* ret = instance->data();
			return ret;
	}

	// cv::fromSlice(Indirect, Primitive) generated
	// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const size_t*", "size_t"]), _)]),
	std::vector<size_t>* cv_fromSlice_const_const_size_tX_size_t(const size_t* data, size_t len) {
			return new std::vector<size_t>(data, data + len);
	}

}


