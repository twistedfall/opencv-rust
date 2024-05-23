extern "C" {
	// std::vector<std::vector<signed char>>::new() generated
	// ("std::vector<std::vector<signed char>>::new", vec![(pred!(const, [], []), _)]),
	std::vector<std::vector<signed char>>* std_vectorLstd_vectorLsigned_charGG_new_const() {
			std::vector<std::vector<signed char>>* ret = new std::vector<std::vector<signed char>>();
			return ret;
	}

	// std::vector<std::vector<signed char>>::delete() generated
	// ("std::vector<std::vector<signed char>>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLsigned_charGG_delete(std::vector<std::vector<signed char>>* instance) {
			delete instance;
	}

	// std::vector<std::vector<signed char>>::len() generated
	// ("std::vector<std::vector<signed char>>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_vectorLsigned_charGG_len_const(const std::vector<std::vector<signed char>>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<std::vector<signed char>>::isEmpty() generated
	// ("std::vector<std::vector<signed char>>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLstd_vectorLsigned_charGG_isEmpty_const(const std::vector<std::vector<signed char>>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<std::vector<signed char>>::capacity() generated
	// ("std::vector<std::vector<signed char>>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_vectorLsigned_charGG_capacity_const(const std::vector<std::vector<signed char>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<std::vector<signed char>>::shrinkToFit() generated
	// ("std::vector<std::vector<signed char>>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLsigned_charGG_shrinkToFit(std::vector<std::vector<signed char>>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<std::vector<signed char>>::reserve(Primitive) generated
	// ("std::vector<std::vector<signed char>>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLsigned_charGG_reserve_size_t(std::vector<std::vector<signed char>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<std::vector<signed char>>::remove(Primitive) generated
	// ("std::vector<std::vector<signed char>>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLsigned_charGG_remove_size_t(std::vector<std::vector<signed char>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<std::vector<signed char>>::swap(Primitive, Primitive) generated
	// ("std::vector<std::vector<signed char>>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLstd_vectorLsigned_charGG_swap_size_t_size_t(std::vector<std::vector<signed char>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<std::vector<signed char>>::clear() generated
	// ("std::vector<std::vector<signed char>>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLsigned_charGG_clear(std::vector<std::vector<signed char>>* instance) {
			instance->clear();
	}

	// std::vector<std::vector<signed char>>::push(CppPassByVoidPtr) generated
	// ("std::vector<std::vector<signed char>>::push", vec![(pred!(mut, ["val"], ["const std::vector<signed char>"]), _)]),
	void std_vectorLstd_vectorLsigned_charGG_push_const_vectorLsigned_charG(std::vector<std::vector<signed char>>* instance, const std::vector<signed char>* val) {
			instance->push_back(*val);
	}

	// std::vector<std::vector<signed char>>::insert(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::vector<signed char>>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::vector<signed char>"]), _)]),
	void std_vectorLstd_vectorLsigned_charGG_insert_size_t_const_vectorLsigned_charG(std::vector<std::vector<signed char>>* instance, size_t index, const std::vector<signed char>* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<std::vector<signed char>>::get(Primitive) generated
	// ("std::vector<std::vector<signed char>>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLsigned_charGG_get_const_size_t(const std::vector<std::vector<signed char>>* instance, size_t index, std::vector<signed char>** ocvrs_return) {
			std::vector<signed char> ret = (*instance)[index];
			*ocvrs_return = new std::vector<signed char>(ret);
	}

	// std::vector<std::vector<signed char>>::set(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::vector<signed char>>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::vector<signed char>"]), _)]),
	void std_vectorLstd_vectorLsigned_charGG_set_size_t_const_vectorLsigned_charG(std::vector<std::vector<signed char>>* instance, size_t index, const std::vector<signed char>* val) {
			(*instance)[index] = *val;
	}

}


