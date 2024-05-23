extern "C" {
	// std::vector<bool>::new() generated
	// ("std::vector<bool>::new", vec![(pred!(const, [], []), _)]),
	std::vector<bool>* std_vectorLboolG_new_const() {
			std::vector<bool>* ret = new std::vector<bool>();
			return ret;
	}

	// std::vector<bool>::delete() generated
	// ("std::vector<bool>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLboolG_delete(std::vector<bool>* instance) {
			delete instance;
	}

	// std::vector<bool>::len() generated
	// ("std::vector<bool>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLboolG_len_const(const std::vector<bool>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<bool>::isEmpty() generated
	// ("std::vector<bool>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLboolG_isEmpty_const(const std::vector<bool>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<bool>::capacity() generated
	// ("std::vector<bool>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLboolG_capacity_const(const std::vector<bool>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<bool>::shrinkToFit() generated
	// ("std::vector<bool>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLboolG_shrinkToFit(std::vector<bool>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<bool>::reserve(Primitive) generated
	// ("std::vector<bool>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLboolG_reserve_size_t(std::vector<bool>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<bool>::remove(Primitive) generated
	// ("std::vector<bool>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLboolG_remove_size_t(std::vector<bool>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<bool>::swap(Primitive, Primitive) generated
	// ("std::vector<bool>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLboolG_swap_size_t_size_t(std::vector<bool>* instance, size_t index1, size_t index2) {
			instance->swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<bool>::clear() generated
	// ("std::vector<bool>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLboolG_clear(std::vector<bool>* instance) {
			instance->clear();
	}

	// std::vector<bool>::push(Primitive) generated
	// ("std::vector<bool>::push", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void std_vectorLboolG_push_const_bool(std::vector<bool>* instance, const bool val) {
			instance->push_back(val);
	}

	// std::vector<bool>::insert(Primitive, Primitive) generated
	// ("std::vector<bool>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const bool"]), _)]),
	void std_vectorLboolG_insert_size_t_const_bool(std::vector<bool>* instance, size_t index, const bool val) {
			instance->insert(instance->begin() + index, val);
	}

	// std::vector<bool>::get(Primitive) generated
	// ("std::vector<bool>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLboolG_get_const_size_t(const std::vector<bool>* instance, size_t index, bool* ocvrs_return) {
			bool ret = (*instance)[index];
			*ocvrs_return = ret;
	}

	// std::vector<bool>::set(Primitive, Primitive) generated
	// ("std::vector<bool>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const bool"]), _)]),
	void std_vectorLboolG_set_size_t_const_bool(std::vector<bool>* instance, size_t index, const bool val) {
			(*instance)[index] = val;
	}

	// std::vector<bool>::inputArray() generated
	// ("std::vector<bool>::inputArray", vec![(pred!(const, [], []), _)]),
	void std_vectorLboolG_inputArray_const(const std::vector<bool>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			const cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new const cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

}


