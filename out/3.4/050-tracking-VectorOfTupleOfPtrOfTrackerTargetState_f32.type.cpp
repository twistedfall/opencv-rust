extern "C" {
	// std::vector<std::pair<cv::Ptr<cv::TrackerTargetState>, float>>::new() generated
	// ("std::vector<std::pair<cv::Ptr<cv::TrackerTargetState>, float>>::new", vec![(pred!(const, [], []), _)]),
	std::vector<std::pair<cv::Ptr<cv::TrackerTargetState>, float>>* std_vectorLstd_pairLcv_PtrLcv_TrackerTargetStateG__floatGG_new_const() {
			std::vector<std::pair<cv::Ptr<cv::TrackerTargetState>, float>>* ret = new std::vector<std::pair<cv::Ptr<cv::TrackerTargetState>, float>>();
			return ret;
	}

	// std::vector<std::pair<cv::Ptr<cv::TrackerTargetState>, float>>::delete() generated
	// ("std::vector<std::pair<cv::Ptr<cv::TrackerTargetState>, float>>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_pairLcv_PtrLcv_TrackerTargetStateG__floatGG_delete(std::vector<std::pair<cv::Ptr<cv::TrackerTargetState>, float>>* instance) {
			delete instance;
	}

	// std::vector<std::pair<cv::Ptr<cv::TrackerTargetState>, float>>::len() generated
	// ("std::vector<std::pair<cv::Ptr<cv::TrackerTargetState>, float>>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_pairLcv_PtrLcv_TrackerTargetStateG__floatGG_len_const(const std::vector<std::pair<cv::Ptr<cv::TrackerTargetState>, float>>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<std::pair<cv::Ptr<cv::TrackerTargetState>, float>>::isEmpty() generated
	// ("std::vector<std::pair<cv::Ptr<cv::TrackerTargetState>, float>>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLstd_pairLcv_PtrLcv_TrackerTargetStateG__floatGG_isEmpty_const(const std::vector<std::pair<cv::Ptr<cv::TrackerTargetState>, float>>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<std::pair<cv::Ptr<cv::TrackerTargetState>, float>>::capacity() generated
	// ("std::vector<std::pair<cv::Ptr<cv::TrackerTargetState>, float>>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_pairLcv_PtrLcv_TrackerTargetStateG__floatGG_capacity_const(const std::vector<std::pair<cv::Ptr<cv::TrackerTargetState>, float>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<std::pair<cv::Ptr<cv::TrackerTargetState>, float>>::shrinkToFit() generated
	// ("std::vector<std::pair<cv::Ptr<cv::TrackerTargetState>, float>>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_pairLcv_PtrLcv_TrackerTargetStateG__floatGG_shrinkToFit(std::vector<std::pair<cv::Ptr<cv::TrackerTargetState>, float>>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<std::pair<cv::Ptr<cv::TrackerTargetState>, float>>::reserve(Primitive) generated
	// ("std::vector<std::pair<cv::Ptr<cv::TrackerTargetState>, float>>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLstd_pairLcv_PtrLcv_TrackerTargetStateG__floatGG_reserve_size_t(std::vector<std::pair<cv::Ptr<cv::TrackerTargetState>, float>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<std::pair<cv::Ptr<cv::TrackerTargetState>, float>>::remove(Primitive) generated
	// ("std::vector<std::pair<cv::Ptr<cv::TrackerTargetState>, float>>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_pairLcv_PtrLcv_TrackerTargetStateG__floatGG_remove_size_t(std::vector<std::pair<cv::Ptr<cv::TrackerTargetState>, float>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<std::pair<cv::Ptr<cv::TrackerTargetState>, float>>::swap(Primitive, Primitive) generated
	// ("std::vector<std::pair<cv::Ptr<cv::TrackerTargetState>, float>>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLstd_pairLcv_PtrLcv_TrackerTargetStateG__floatGG_swap_size_t_size_t(std::vector<std::pair<cv::Ptr<cv::TrackerTargetState>, float>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<std::pair<cv::Ptr<cv::TrackerTargetState>, float>>::clear() generated
	// ("std::vector<std::pair<cv::Ptr<cv::TrackerTargetState>, float>>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_pairLcv_PtrLcv_TrackerTargetStateG__floatGG_clear(std::vector<std::pair<cv::Ptr<cv::TrackerTargetState>, float>>* instance) {
			instance->clear();
	}

	// std::vector<std::pair<cv::Ptr<cv::TrackerTargetState>, float>>::push(CppPassByVoidPtr) generated
	// ("std::vector<std::pair<cv::Ptr<cv::TrackerTargetState>, float>>::push", vec![(pred!(mut, ["val"], ["const std::pair<cv::Ptr<cv::TrackerTargetState>, float>"]), _)]),
	void std_vectorLstd_pairLcv_PtrLcv_TrackerTargetStateG__floatGG_push_const_pairLcv_PtrLcv_TrackerTargetStateG__floatG(std::vector<std::pair<cv::Ptr<cv::TrackerTargetState>, float>>* instance, const std::pair<cv::Ptr<cv::TrackerTargetState>, float>* val) {
			instance->push_back(*val);
	}

	// std::vector<std::pair<cv::Ptr<cv::TrackerTargetState>, float>>::insert(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::pair<cv::Ptr<cv::TrackerTargetState>, float>>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::pair<cv::Ptr<cv::TrackerTargetState>, float>"]), _)]),
	void std_vectorLstd_pairLcv_PtrLcv_TrackerTargetStateG__floatGG_insert_size_t_const_pairLcv_PtrLcv_TrackerTargetStateG__floatG(std::vector<std::pair<cv::Ptr<cv::TrackerTargetState>, float>>* instance, size_t index, const std::pair<cv::Ptr<cv::TrackerTargetState>, float>* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<std::pair<cv::Ptr<cv::TrackerTargetState>, float>>::get(Primitive) generated
	// ("std::vector<std::pair<cv::Ptr<cv::TrackerTargetState>, float>>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_pairLcv_PtrLcv_TrackerTargetStateG__floatGG_get_const_size_t(const std::vector<std::pair<cv::Ptr<cv::TrackerTargetState>, float>>* instance, size_t index, std::pair<cv::Ptr<cv::TrackerTargetState>, float>** ocvrs_return) {
			std::pair<cv::Ptr<cv::TrackerTargetState>, float> ret = (*instance)[index];
			*ocvrs_return = new std::pair<cv::Ptr<cv::TrackerTargetState>, float>(ret);
	}

	// std::vector<std::pair<cv::Ptr<cv::TrackerTargetState>, float>>::set(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::pair<cv::Ptr<cv::TrackerTargetState>, float>>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::pair<cv::Ptr<cv::TrackerTargetState>, float>"]), _)]),
	void std_vectorLstd_pairLcv_PtrLcv_TrackerTargetStateG__floatGG_set_size_t_const_pairLcv_PtrLcv_TrackerTargetStateG__floatG(std::vector<std::pair<cv::Ptr<cv::TrackerTargetState>, float>>* instance, size_t index, const std::pair<cv::Ptr<cv::TrackerTargetState>, float>* val) {
			(*instance)[index] = *val;
	}

}


