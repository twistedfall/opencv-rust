extern "C" {
	// std::vector<cv::DetectionBasedTracker::Object>::new() generated
	// ("std::vector<cv::DetectionBasedTracker::Object>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::DetectionBasedTracker::Object>* std_vectorLcv_DetectionBasedTracker_ObjectG_new_const() {
			std::vector<cv::DetectionBasedTracker::Object>* ret = new std::vector<cv::DetectionBasedTracker::Object>();
			return ret;
	}

	// std::vector<cv::DetectionBasedTracker::Object>::delete() generated
	// ("std::vector<cv::DetectionBasedTracker::Object>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_DetectionBasedTracker_ObjectG_delete(std::vector<cv::DetectionBasedTracker::Object>* instance) {
			delete instance;
	}

	// std::vector<cv::DetectionBasedTracker::Object>::len() generated
	// ("std::vector<cv::DetectionBasedTracker::Object>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_DetectionBasedTracker_ObjectG_len_const(const std::vector<cv::DetectionBasedTracker::Object>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::DetectionBasedTracker::Object>::isEmpty() generated
	// ("std::vector<cv::DetectionBasedTracker::Object>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_DetectionBasedTracker_ObjectG_isEmpty_const(const std::vector<cv::DetectionBasedTracker::Object>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::DetectionBasedTracker::Object>::capacity() generated
	// ("std::vector<cv::DetectionBasedTracker::Object>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_DetectionBasedTracker_ObjectG_capacity_const(const std::vector<cv::DetectionBasedTracker::Object>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::DetectionBasedTracker::Object>::shrinkToFit() generated
	// ("std::vector<cv::DetectionBasedTracker::Object>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_DetectionBasedTracker_ObjectG_shrinkToFit(std::vector<cv::DetectionBasedTracker::Object>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::DetectionBasedTracker::Object>::reserve(Primitive) generated
	// ("std::vector<cv::DetectionBasedTracker::Object>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_DetectionBasedTracker_ObjectG_reserve_size_t(std::vector<cv::DetectionBasedTracker::Object>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::DetectionBasedTracker::Object>::remove(Primitive) generated
	// ("std::vector<cv::DetectionBasedTracker::Object>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_DetectionBasedTracker_ObjectG_remove_size_t(std::vector<cv::DetectionBasedTracker::Object>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::DetectionBasedTracker::Object>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::DetectionBasedTracker::Object>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_DetectionBasedTracker_ObjectG_swap_size_t_size_t(std::vector<cv::DetectionBasedTracker::Object>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::DetectionBasedTracker::Object>::clear() generated
	// ("std::vector<cv::DetectionBasedTracker::Object>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_DetectionBasedTracker_ObjectG_clear(std::vector<cv::DetectionBasedTracker::Object>* instance) {
			instance->clear();
	}

	// std::vector<cv::DetectionBasedTracker::Object>::push(CppPassByVoidPtr) generated
	// ("std::vector<cv::DetectionBasedTracker::Object>::push", vec![(pred!(mut, ["val"], ["const cv::DetectionBasedTracker::Object"]), _)]),
	void std_vectorLcv_DetectionBasedTracker_ObjectG_push_const_Object(std::vector<cv::DetectionBasedTracker::Object>* instance, const cv::DetectionBasedTracker::Object* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::DetectionBasedTracker::Object>::insert(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<cv::DetectionBasedTracker::Object>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::DetectionBasedTracker::Object"]), _)]),
	void std_vectorLcv_DetectionBasedTracker_ObjectG_insert_size_t_const_Object(std::vector<cv::DetectionBasedTracker::Object>* instance, size_t index, const cv::DetectionBasedTracker::Object* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::DetectionBasedTracker::Object>::get(Primitive) generated
	// ("std::vector<cv::DetectionBasedTracker::Object>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_DetectionBasedTracker_ObjectG_get_const_size_t(const std::vector<cv::DetectionBasedTracker::Object>* instance, size_t index, cv::DetectionBasedTracker::Object** ocvrs_return) {
			cv::DetectionBasedTracker::Object ret = (*instance)[index];
			*ocvrs_return = new cv::DetectionBasedTracker::Object(ret);
	}

	// std::vector<cv::DetectionBasedTracker::Object>::set(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<cv::DetectionBasedTracker::Object>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::DetectionBasedTracker::Object"]), _)]),
	void std_vectorLcv_DetectionBasedTracker_ObjectG_set_size_t_const_Object(std::vector<cv::DetectionBasedTracker::Object>* instance, size_t index, const cv::DetectionBasedTracker::Object* val) {
			(*instance)[index] = *val;
	}

}


