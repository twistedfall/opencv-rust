extern "C" {
	// std::vector<std::pair<int, double>>::new() generated
	// ("std::vector<std::pair<int, double>>::new", vec![(pred!(const, [], []), _)]),
	std::vector<std::pair<int, double>>* std_vectorLstd_pairLint__doubleGG_new_const() {
			std::vector<std::pair<int, double>>* ret = new std::vector<std::pair<int, double>>();
			return ret;
	}

	// std::vector<std::pair<int, double>>::delete() generated
	// ("std::vector<std::pair<int, double>>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_pairLint__doubleGG_delete(std::vector<std::pair<int, double>>* instance) {
			delete instance;
	}

	// std::vector<std::pair<int, double>>::len() generated
	// ("std::vector<std::pair<int, double>>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_pairLint__doubleGG_len_const(const std::vector<std::pair<int, double>>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<std::pair<int, double>>::isEmpty() generated
	// ("std::vector<std::pair<int, double>>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLstd_pairLint__doubleGG_isEmpty_const(const std::vector<std::pair<int, double>>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<std::pair<int, double>>::capacity() generated
	// ("std::vector<std::pair<int, double>>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_pairLint__doubleGG_capacity_const(const std::vector<std::pair<int, double>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<std::pair<int, double>>::shrinkToFit() generated
	// ("std::vector<std::pair<int, double>>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_pairLint__doubleGG_shrinkToFit(std::vector<std::pair<int, double>>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<std::pair<int, double>>::reserve(Primitive) generated
	// ("std::vector<std::pair<int, double>>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLstd_pairLint__doubleGG_reserve_size_t(std::vector<std::pair<int, double>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<std::pair<int, double>>::remove(Primitive) generated
	// ("std::vector<std::pair<int, double>>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_pairLint__doubleGG_remove_size_t(std::vector<std::pair<int, double>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<std::pair<int, double>>::swap(Primitive, Primitive) generated
	// ("std::vector<std::pair<int, double>>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLstd_pairLint__doubleGG_swap_size_t_size_t(std::vector<std::pair<int, double>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<std::pair<int, double>>::clear() generated
	// ("std::vector<std::pair<int, double>>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_pairLint__doubleGG_clear(std::vector<std::pair<int, double>>* instance) {
			instance->clear();
	}

	// std::vector<std::pair<int, double>>::push(CppPassByVoidPtr) generated
	// ("std::vector<std::pair<int, double>>::push", vec![(pred!(mut, ["val"], ["const std::pair<int, double>"]), _)]),
	void std_vectorLstd_pairLint__doubleGG_push_const_pairLint__doubleG(std::vector<std::pair<int, double>>* instance, const std::pair<int, double>* val) {
			instance->push_back(*val);
	}

	// std::vector<std::pair<int, double>>::insert(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::pair<int, double>>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::pair<int, double>"]), _)]),
	void std_vectorLstd_pairLint__doubleGG_insert_size_t_const_pairLint__doubleG(std::vector<std::pair<int, double>>* instance, size_t index, const std::pair<int, double>* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<std::pair<int, double>>::get(Primitive) generated
	// ("std::vector<std::pair<int, double>>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_pairLint__doubleGG_get_const_size_t(const std::vector<std::pair<int, double>>* instance, size_t index, std::pair<int, double>** ocvrs_return) {
			std::pair<int, double> ret = (*instance)[index];
			*ocvrs_return = new std::pair<int, double>(ret);
	}

	// std::vector<std::pair<int, double>>::set(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::pair<int, double>>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::pair<int, double>"]), _)]),
	void std_vectorLstd_pairLint__doubleGG_set_size_t_const_pairLint__doubleG(std::vector<std::pair<int, double>>* instance, size_t index, const std::pair<int, double>* val) {
			(*instance)[index] = *val;
	}

}


