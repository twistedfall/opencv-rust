extern "C" {
	// std::vector<cv::DetectionBasedTracker::ExtObject>::new() generated
	// ("std::vector<cv::DetectionBasedTracker::ExtObject>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::DetectionBasedTracker::ExtObject>* std_vectorLcv_DetectionBasedTracker_ExtObjectG_new_const() {
			std::vector<cv::DetectionBasedTracker::ExtObject>* ret = new std::vector<cv::DetectionBasedTracker::ExtObject>();
			return ret;
	}

	// std::vector<cv::DetectionBasedTracker::ExtObject>::delete() generated
	// ("std::vector<cv::DetectionBasedTracker::ExtObject>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_DetectionBasedTracker_ExtObjectG_delete(std::vector<cv::DetectionBasedTracker::ExtObject>* instance) {
			delete instance;
	}

	// std::vector<cv::DetectionBasedTracker::ExtObject>::len() generated
	// ("std::vector<cv::DetectionBasedTracker::ExtObject>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_DetectionBasedTracker_ExtObjectG_len_const(const std::vector<cv::DetectionBasedTracker::ExtObject>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::DetectionBasedTracker::ExtObject>::isEmpty() generated
	// ("std::vector<cv::DetectionBasedTracker::ExtObject>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_DetectionBasedTracker_ExtObjectG_isEmpty_const(const std::vector<cv::DetectionBasedTracker::ExtObject>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::DetectionBasedTracker::ExtObject>::capacity() generated
	// ("std::vector<cv::DetectionBasedTracker::ExtObject>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_DetectionBasedTracker_ExtObjectG_capacity_const(const std::vector<cv::DetectionBasedTracker::ExtObject>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::DetectionBasedTracker::ExtObject>::shrinkToFit() generated
	// ("std::vector<cv::DetectionBasedTracker::ExtObject>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_DetectionBasedTracker_ExtObjectG_shrinkToFit(std::vector<cv::DetectionBasedTracker::ExtObject>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::DetectionBasedTracker::ExtObject>::reserve(Primitive) generated
	// ("std::vector<cv::DetectionBasedTracker::ExtObject>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_DetectionBasedTracker_ExtObjectG_reserve_size_t(std::vector<cv::DetectionBasedTracker::ExtObject>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::DetectionBasedTracker::ExtObject>::remove(Primitive) generated
	// ("std::vector<cv::DetectionBasedTracker::ExtObject>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_DetectionBasedTracker_ExtObjectG_remove_size_t(std::vector<cv::DetectionBasedTracker::ExtObject>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::DetectionBasedTracker::ExtObject>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::DetectionBasedTracker::ExtObject>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_DetectionBasedTracker_ExtObjectG_swap_size_t_size_t(std::vector<cv::DetectionBasedTracker::ExtObject>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::DetectionBasedTracker::ExtObject>::clear() generated
	// ("std::vector<cv::DetectionBasedTracker::ExtObject>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_DetectionBasedTracker_ExtObjectG_clear(std::vector<cv::DetectionBasedTracker::ExtObject>* instance) {
			instance->clear();
	}

	// std::vector<cv::DetectionBasedTracker::ExtObject>::push(TraitClass) generated
	// ("std::vector<cv::DetectionBasedTracker::ExtObject>::push", vec![(pred!(mut, ["val"], ["const cv::DetectionBasedTracker::ExtObject"]), _)]),
	void std_vectorLcv_DetectionBasedTracker_ExtObjectG_push_const_ExtObject(std::vector<cv::DetectionBasedTracker::ExtObject>* instance, const cv::DetectionBasedTracker::ExtObject* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::DetectionBasedTracker::ExtObject>::insert(Primitive, TraitClass) generated
	// ("std::vector<cv::DetectionBasedTracker::ExtObject>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::DetectionBasedTracker::ExtObject"]), _)]),
	void std_vectorLcv_DetectionBasedTracker_ExtObjectG_insert_size_t_const_ExtObject(std::vector<cv::DetectionBasedTracker::ExtObject>* instance, size_t index, const cv::DetectionBasedTracker::ExtObject* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::DetectionBasedTracker::ExtObject>::get(Primitive) generated
	// ("std::vector<cv::DetectionBasedTracker::ExtObject>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_DetectionBasedTracker_ExtObjectG_get_const_size_t(const std::vector<cv::DetectionBasedTracker::ExtObject>* instance, size_t index, cv::DetectionBasedTracker::ExtObject** ocvrs_return) {
			cv::DetectionBasedTracker::ExtObject ret = (*instance)[index];
			*ocvrs_return = new cv::DetectionBasedTracker::ExtObject(ret);
	}

	// std::vector<cv::DetectionBasedTracker::ExtObject>::set(Primitive, TraitClass) generated
	// ("std::vector<cv::DetectionBasedTracker::ExtObject>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::DetectionBasedTracker::ExtObject"]), _)]),
	void std_vectorLcv_DetectionBasedTracker_ExtObjectG_set_size_t_const_ExtObject(std::vector<cv::DetectionBasedTracker::ExtObject>* instance, size_t index, const cv::DetectionBasedTracker::ExtObject* val) {
			(*instance)[index] = *val;
	}

}


