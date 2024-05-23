extern "C" {
	// std::vector<cv::Ptr<cv::mcc::CChecker>>::new() generated
	// ("std::vector<cv::Ptr<cv::mcc::CChecker>>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Ptr<cv::mcc::CChecker>>* std_vectorLcv_PtrLcv_mcc_CCheckerGG_new_const() {
			std::vector<cv::Ptr<cv::mcc::CChecker>>* ret = new std::vector<cv::Ptr<cv::mcc::CChecker>>();
			return ret;
	}

	// std::vector<cv::Ptr<cv::mcc::CChecker>>::delete() generated
	// ("std::vector<cv::Ptr<cv::mcc::CChecker>>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_PtrLcv_mcc_CCheckerGG_delete(std::vector<cv::Ptr<cv::mcc::CChecker>>* instance) {
			delete instance;
	}

	// std::vector<cv::Ptr<cv::mcc::CChecker>>::len() generated
	// ("std::vector<cv::Ptr<cv::mcc::CChecker>>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_PtrLcv_mcc_CCheckerGG_len_const(const std::vector<cv::Ptr<cv::mcc::CChecker>>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::Ptr<cv::mcc::CChecker>>::isEmpty() generated
	// ("std::vector<cv::Ptr<cv::mcc::CChecker>>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_PtrLcv_mcc_CCheckerGG_isEmpty_const(const std::vector<cv::Ptr<cv::mcc::CChecker>>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::Ptr<cv::mcc::CChecker>>::capacity() generated
	// ("std::vector<cv::Ptr<cv::mcc::CChecker>>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_PtrLcv_mcc_CCheckerGG_capacity_const(const std::vector<cv::Ptr<cv::mcc::CChecker>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::Ptr<cv::mcc::CChecker>>::shrinkToFit() generated
	// ("std::vector<cv::Ptr<cv::mcc::CChecker>>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_PtrLcv_mcc_CCheckerGG_shrinkToFit(std::vector<cv::Ptr<cv::mcc::CChecker>>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::Ptr<cv::mcc::CChecker>>::reserve(Primitive) generated
	// ("std::vector<cv::Ptr<cv::mcc::CChecker>>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_PtrLcv_mcc_CCheckerGG_reserve_size_t(std::vector<cv::Ptr<cv::mcc::CChecker>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::Ptr<cv::mcc::CChecker>>::remove(Primitive) generated
	// ("std::vector<cv::Ptr<cv::mcc::CChecker>>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_PtrLcv_mcc_CCheckerGG_remove_size_t(std::vector<cv::Ptr<cv::mcc::CChecker>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::Ptr<cv::mcc::CChecker>>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::Ptr<cv::mcc::CChecker>>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_PtrLcv_mcc_CCheckerGG_swap_size_t_size_t(std::vector<cv::Ptr<cv::mcc::CChecker>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::Ptr<cv::mcc::CChecker>>::clear() generated
	// ("std::vector<cv::Ptr<cv::mcc::CChecker>>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_PtrLcv_mcc_CCheckerGG_clear(std::vector<cv::Ptr<cv::mcc::CChecker>>* instance) {
			instance->clear();
	}

	// std::vector<cv::Ptr<cv::mcc::CChecker>>::push(CppPassByVoidPtr) generated
	// ("std::vector<cv::Ptr<cv::mcc::CChecker>>::push", vec![(pred!(mut, ["val"], ["const cv::Ptr<cv::mcc::CChecker>"]), _)]),
	void std_vectorLcv_PtrLcv_mcc_CCheckerGG_push_const_PtrLCCheckerG(std::vector<cv::Ptr<cv::mcc::CChecker>>* instance, const cv::Ptr<cv::mcc::CChecker>* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::Ptr<cv::mcc::CChecker>>::insert(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<cv::Ptr<cv::mcc::CChecker>>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Ptr<cv::mcc::CChecker>"]), _)]),
	void std_vectorLcv_PtrLcv_mcc_CCheckerGG_insert_size_t_const_PtrLCCheckerG(std::vector<cv::Ptr<cv::mcc::CChecker>>* instance, size_t index, const cv::Ptr<cv::mcc::CChecker>* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::Ptr<cv::mcc::CChecker>>::get(Primitive) generated
	// ("std::vector<cv::Ptr<cv::mcc::CChecker>>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_PtrLcv_mcc_CCheckerGG_get_const_size_t(const std::vector<cv::Ptr<cv::mcc::CChecker>>* instance, size_t index, cv::Ptr<cv::mcc::CChecker>** ocvrs_return) {
			cv::Ptr<cv::mcc::CChecker> ret = (*instance)[index];
			*ocvrs_return = new cv::Ptr<cv::mcc::CChecker>(ret);
	}

	// std::vector<cv::Ptr<cv::mcc::CChecker>>::set(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<cv::Ptr<cv::mcc::CChecker>>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Ptr<cv::mcc::CChecker>"]), _)]),
	void std_vectorLcv_PtrLcv_mcc_CCheckerGG_set_size_t_const_PtrLCCheckerG(std::vector<cv::Ptr<cv::mcc::CChecker>>* instance, size_t index, const cv::Ptr<cv::mcc::CChecker>* val) {
			(*instance)[index] = *val;
	}

}


