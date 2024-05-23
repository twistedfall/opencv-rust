extern "C" {
	// std::vector<cv::Ptr<cv::TrackerTargetState>>::new() generated
	// ("std::vector<cv::Ptr<cv::TrackerTargetState>>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Ptr<cv::TrackerTargetState>>* std_vectorLcv_PtrLcv_TrackerTargetStateGG_new_const() {
			std::vector<cv::Ptr<cv::TrackerTargetState>>* ret = new std::vector<cv::Ptr<cv::TrackerTargetState>>();
			return ret;
	}

	// std::vector<cv::Ptr<cv::TrackerTargetState>>::delete() generated
	// ("std::vector<cv::Ptr<cv::TrackerTargetState>>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_PtrLcv_TrackerTargetStateGG_delete(std::vector<cv::Ptr<cv::TrackerTargetState>>* instance) {
			delete instance;
	}

	// std::vector<cv::Ptr<cv::TrackerTargetState>>::len() generated
	// ("std::vector<cv::Ptr<cv::TrackerTargetState>>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_PtrLcv_TrackerTargetStateGG_len_const(const std::vector<cv::Ptr<cv::TrackerTargetState>>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::Ptr<cv::TrackerTargetState>>::isEmpty() generated
	// ("std::vector<cv::Ptr<cv::TrackerTargetState>>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_PtrLcv_TrackerTargetStateGG_isEmpty_const(const std::vector<cv::Ptr<cv::TrackerTargetState>>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::Ptr<cv::TrackerTargetState>>::capacity() generated
	// ("std::vector<cv::Ptr<cv::TrackerTargetState>>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_PtrLcv_TrackerTargetStateGG_capacity_const(const std::vector<cv::Ptr<cv::TrackerTargetState>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::Ptr<cv::TrackerTargetState>>::shrinkToFit() generated
	// ("std::vector<cv::Ptr<cv::TrackerTargetState>>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_PtrLcv_TrackerTargetStateGG_shrinkToFit(std::vector<cv::Ptr<cv::TrackerTargetState>>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::Ptr<cv::TrackerTargetState>>::reserve(Primitive) generated
	// ("std::vector<cv::Ptr<cv::TrackerTargetState>>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_PtrLcv_TrackerTargetStateGG_reserve_size_t(std::vector<cv::Ptr<cv::TrackerTargetState>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::Ptr<cv::TrackerTargetState>>::remove(Primitive) generated
	// ("std::vector<cv::Ptr<cv::TrackerTargetState>>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_PtrLcv_TrackerTargetStateGG_remove_size_t(std::vector<cv::Ptr<cv::TrackerTargetState>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::Ptr<cv::TrackerTargetState>>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::Ptr<cv::TrackerTargetState>>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_PtrLcv_TrackerTargetStateGG_swap_size_t_size_t(std::vector<cv::Ptr<cv::TrackerTargetState>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::Ptr<cv::TrackerTargetState>>::clear() generated
	// ("std::vector<cv::Ptr<cv::TrackerTargetState>>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_PtrLcv_TrackerTargetStateGG_clear(std::vector<cv::Ptr<cv::TrackerTargetState>>* instance) {
			instance->clear();
	}

	// std::vector<cv::Ptr<cv::TrackerTargetState>>::push(CppPassByVoidPtr) generated
	// ("std::vector<cv::Ptr<cv::TrackerTargetState>>::push", vec![(pred!(mut, ["val"], ["const cv::Ptr<cv::TrackerTargetState>"]), _)]),
	void std_vectorLcv_PtrLcv_TrackerTargetStateGG_push_const_PtrLTrackerTargetStateG(std::vector<cv::Ptr<cv::TrackerTargetState>>* instance, const cv::Ptr<cv::TrackerTargetState>* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::Ptr<cv::TrackerTargetState>>::insert(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<cv::Ptr<cv::TrackerTargetState>>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Ptr<cv::TrackerTargetState>"]), _)]),
	void std_vectorLcv_PtrLcv_TrackerTargetStateGG_insert_size_t_const_PtrLTrackerTargetStateG(std::vector<cv::Ptr<cv::TrackerTargetState>>* instance, size_t index, const cv::Ptr<cv::TrackerTargetState>* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::Ptr<cv::TrackerTargetState>>::get(Primitive) generated
	// ("std::vector<cv::Ptr<cv::TrackerTargetState>>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_PtrLcv_TrackerTargetStateGG_get_const_size_t(const std::vector<cv::Ptr<cv::TrackerTargetState>>* instance, size_t index, cv::Ptr<cv::TrackerTargetState>** ocvrs_return) {
			cv::Ptr<cv::TrackerTargetState> ret = (*instance)[index];
			*ocvrs_return = new cv::Ptr<cv::TrackerTargetState>(ret);
	}

	// std::vector<cv::Ptr<cv::TrackerTargetState>>::set(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<cv::Ptr<cv::TrackerTargetState>>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Ptr<cv::TrackerTargetState>"]), _)]),
	void std_vectorLcv_PtrLcv_TrackerTargetStateGG_set_size_t_const_PtrLTrackerTargetStateG(std::vector<cv::Ptr<cv::TrackerTargetState>>* instance, size_t index, const cv::Ptr<cv::TrackerTargetState>* val) {
			(*instance)[index] = *val;
	}

}


