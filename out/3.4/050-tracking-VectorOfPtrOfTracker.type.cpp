extern "C" {
	// std::vector<cv::Ptr<cv::Tracker>>::new() generated
	// ("std::vector<cv::Ptr<cv::Tracker>>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Ptr<cv::Tracker>>* std_vectorLcv_PtrLcv_TrackerGG_new_const() {
			std::vector<cv::Ptr<cv::Tracker>>* ret = new std::vector<cv::Ptr<cv::Tracker>>();
			return ret;
	}

	// std::vector<cv::Ptr<cv::Tracker>>::delete() generated
	// ("std::vector<cv::Ptr<cv::Tracker>>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_PtrLcv_TrackerGG_delete(std::vector<cv::Ptr<cv::Tracker>>* instance) {
			delete instance;
	}

	// std::vector<cv::Ptr<cv::Tracker>>::len() generated
	// ("std::vector<cv::Ptr<cv::Tracker>>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_PtrLcv_TrackerGG_len_const(const std::vector<cv::Ptr<cv::Tracker>>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::Ptr<cv::Tracker>>::isEmpty() generated
	// ("std::vector<cv::Ptr<cv::Tracker>>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_PtrLcv_TrackerGG_isEmpty_const(const std::vector<cv::Ptr<cv::Tracker>>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::Ptr<cv::Tracker>>::capacity() generated
	// ("std::vector<cv::Ptr<cv::Tracker>>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_PtrLcv_TrackerGG_capacity_const(const std::vector<cv::Ptr<cv::Tracker>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::Ptr<cv::Tracker>>::shrinkToFit() generated
	// ("std::vector<cv::Ptr<cv::Tracker>>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_PtrLcv_TrackerGG_shrinkToFit(std::vector<cv::Ptr<cv::Tracker>>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::Ptr<cv::Tracker>>::reserve(Primitive) generated
	// ("std::vector<cv::Ptr<cv::Tracker>>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_PtrLcv_TrackerGG_reserve_size_t(std::vector<cv::Ptr<cv::Tracker>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::Ptr<cv::Tracker>>::remove(Primitive) generated
	// ("std::vector<cv::Ptr<cv::Tracker>>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_PtrLcv_TrackerGG_remove_size_t(std::vector<cv::Ptr<cv::Tracker>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::Ptr<cv::Tracker>>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::Ptr<cv::Tracker>>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_PtrLcv_TrackerGG_swap_size_t_size_t(std::vector<cv::Ptr<cv::Tracker>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::Ptr<cv::Tracker>>::clear() generated
	// ("std::vector<cv::Ptr<cv::Tracker>>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_PtrLcv_TrackerGG_clear(std::vector<cv::Ptr<cv::Tracker>>* instance) {
			instance->clear();
	}

	// std::vector<cv::Ptr<cv::Tracker>>::push(CppPassByVoidPtr) generated
	// ("std::vector<cv::Ptr<cv::Tracker>>::push", vec![(pred!(mut, ["val"], ["const cv::Ptr<cv::Tracker>"]), _)]),
	void std_vectorLcv_PtrLcv_TrackerGG_push_const_PtrLTrackerG(std::vector<cv::Ptr<cv::Tracker>>* instance, const cv::Ptr<cv::Tracker>* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::Ptr<cv::Tracker>>::insert(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<cv::Ptr<cv::Tracker>>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Ptr<cv::Tracker>"]), _)]),
	void std_vectorLcv_PtrLcv_TrackerGG_insert_size_t_const_PtrLTrackerG(std::vector<cv::Ptr<cv::Tracker>>* instance, size_t index, const cv::Ptr<cv::Tracker>* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::Ptr<cv::Tracker>>::get(Primitive) generated
	// ("std::vector<cv::Ptr<cv::Tracker>>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_PtrLcv_TrackerGG_get_const_size_t(const std::vector<cv::Ptr<cv::Tracker>>* instance, size_t index, cv::Ptr<cv::Tracker>** ocvrs_return) {
			cv::Ptr<cv::Tracker> ret = (*instance)[index];
			*ocvrs_return = new cv::Ptr<cv::Tracker>(ret);
	}

	// std::vector<cv::Ptr<cv::Tracker>>::set(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<cv::Ptr<cv::Tracker>>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Ptr<cv::Tracker>"]), _)]),
	void std_vectorLcv_PtrLcv_TrackerGG_set_size_t_const_PtrLTrackerG(std::vector<cv::Ptr<cv::Tracker>>* instance, size_t index, const cv::Ptr<cv::Tracker>* val) {
			(*instance)[index] = *val;
	}

}


