extern "C" {
	// std::vector<signed char>::new() generated
	// ("std::vector<signed char>::new", vec![(pred!(const, [], []), _)]),
	std::vector<signed char>* std_vectorLsigned_charG_new_const() {
			std::vector<signed char>* ret = new std::vector<signed char>();
			return ret;
	}

	// std::vector<signed char>::delete() generated
	// ("std::vector<signed char>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLsigned_charG_delete(std::vector<signed char>* instance) {
			delete instance;
	}

	// std::vector<signed char>::len() generated
	// ("std::vector<signed char>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLsigned_charG_len_const(const std::vector<signed char>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<signed char>::isEmpty() generated
	// ("std::vector<signed char>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLsigned_charG_isEmpty_const(const std::vector<signed char>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<signed char>::capacity() generated
	// ("std::vector<signed char>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLsigned_charG_capacity_const(const std::vector<signed char>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<signed char>::shrinkToFit() generated
	// ("std::vector<signed char>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLsigned_charG_shrinkToFit(std::vector<signed char>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<signed char>::reserve(Primitive) generated
	// ("std::vector<signed char>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLsigned_charG_reserve_size_t(std::vector<signed char>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<signed char>::remove(Primitive) generated
	// ("std::vector<signed char>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLsigned_charG_remove_size_t(std::vector<signed char>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<signed char>::swap(Primitive, Primitive) generated
	// ("std::vector<signed char>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLsigned_charG_swap_size_t_size_t(std::vector<signed char>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<signed char>::clear() generated
	// ("std::vector<signed char>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLsigned_charG_clear(std::vector<signed char>* instance) {
			instance->clear();
	}

	// std::vector<signed char>::push(Primitive) generated
	// ("std::vector<signed char>::push", vec![(pred!(mut, ["val"], ["const signed char"]), _)]),
	void std_vectorLsigned_charG_push_const_signed_char(std::vector<signed char>* instance, const signed char val) {
			instance->push_back(val);
	}

	// std::vector<signed char>::insert(Primitive, Primitive) generated
	// ("std::vector<signed char>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const signed char"]), _)]),
	void std_vectorLsigned_charG_insert_size_t_const_signed_char(std::vector<signed char>* instance, size_t index, const signed char val) {
			instance->insert(instance->begin() + index, val);
	}

	// std::vector<signed char>::get(Primitive) generated
	// ("std::vector<signed char>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLsigned_charG_get_const_size_t(const std::vector<signed char>* instance, size_t index, signed char* ocvrs_return) {
			signed char ret = (*instance)[index];
			*ocvrs_return = ret;
	}

	// std::vector<signed char>::set(Primitive, Primitive) generated
	// ("std::vector<signed char>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const signed char"]), _)]),
	void std_vectorLsigned_charG_set_size_t_const_signed_char(std::vector<signed char>* instance, size_t index, const signed char val) {
			(*instance)[index] = val;
	}

	// std::vector<signed char>::clone() generated
	// ("std::vector<signed char>::clone", vec![(pred!(const, [], []), _)]),
	std::vector<signed char>* std_vectorLsigned_charG_clone_const(const std::vector<signed char>* instance) {
			std::vector<signed char> ret = std::vector<signed char>(*instance);
			return new std::vector<signed char>(ret);
	}

	// std::vector<signed char>::data() generated
	// ("std::vector<signed char>::data", vec![(pred!(const, [], []), _)]),
	const signed char* std_vectorLsigned_charG_data_const(const std::vector<signed char>* instance) {
			const signed char* ret = instance->data();
			return ret;
	}

	// std::vector<signed char>::dataMut() generated
	// ("std::vector<signed char>::dataMut", vec![(pred!(mut, [], []), _)]),
	signed char* std_vectorLsigned_charG_dataMut(std::vector<signed char>* instance) {
			signed char* ret = instance->data();
			return ret;
	}

	// cv::fromSlice(Indirect, Primitive) generated
	// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const signed char*", "size_t"]), _)]),
	std::vector<signed char>* cv_fromSlice_const_const_signed_charX_size_t(const signed char* data, size_t len) {
			return new std::vector<signed char>(data, data + len);
	}

}


