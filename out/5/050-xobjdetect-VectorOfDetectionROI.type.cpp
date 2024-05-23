extern "C" {
	// std::vector<cv::DetectionROI>::new() generated
	// ("std::vector<cv::DetectionROI>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::DetectionROI>* std_vectorLcv_DetectionROIG_new_const() {
			std::vector<cv::DetectionROI>* ret = new std::vector<cv::DetectionROI>();
			return ret;
	}

	// std::vector<cv::DetectionROI>::delete() generated
	// ("std::vector<cv::DetectionROI>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_DetectionROIG_delete(std::vector<cv::DetectionROI>* instance) {
			delete instance;
	}

	// std::vector<cv::DetectionROI>::len() generated
	// ("std::vector<cv::DetectionROI>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_DetectionROIG_len_const(const std::vector<cv::DetectionROI>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::DetectionROI>::isEmpty() generated
	// ("std::vector<cv::DetectionROI>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_DetectionROIG_isEmpty_const(const std::vector<cv::DetectionROI>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::DetectionROI>::capacity() generated
	// ("std::vector<cv::DetectionROI>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_DetectionROIG_capacity_const(const std::vector<cv::DetectionROI>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::DetectionROI>::shrinkToFit() generated
	// ("std::vector<cv::DetectionROI>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_DetectionROIG_shrinkToFit(std::vector<cv::DetectionROI>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::DetectionROI>::reserve(Primitive) generated
	// ("std::vector<cv::DetectionROI>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_DetectionROIG_reserve_size_t(std::vector<cv::DetectionROI>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::DetectionROI>::remove(Primitive) generated
	// ("std::vector<cv::DetectionROI>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_DetectionROIG_remove_size_t(std::vector<cv::DetectionROI>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::DetectionROI>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::DetectionROI>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_DetectionROIG_swap_size_t_size_t(std::vector<cv::DetectionROI>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::DetectionROI>::clear() generated
	// ("std::vector<cv::DetectionROI>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_DetectionROIG_clear(std::vector<cv::DetectionROI>* instance) {
			instance->clear();
	}

	// std::vector<cv::DetectionROI>::push(TraitClass) generated
	// ("std::vector<cv::DetectionROI>::push", vec![(pred!(mut, ["val"], ["const cv::DetectionROI"]), _)]),
	void std_vectorLcv_DetectionROIG_push_const_DetectionROI(std::vector<cv::DetectionROI>* instance, const cv::DetectionROI* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::DetectionROI>::insert(Primitive, TraitClass) generated
	// ("std::vector<cv::DetectionROI>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::DetectionROI"]), _)]),
	void std_vectorLcv_DetectionROIG_insert_size_t_const_DetectionROI(std::vector<cv::DetectionROI>* instance, size_t index, const cv::DetectionROI* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::DetectionROI>::get(Primitive) generated
	// ("std::vector<cv::DetectionROI>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_DetectionROIG_get_const_size_t(const std::vector<cv::DetectionROI>* instance, size_t index, cv::DetectionROI** ocvrs_return) {
			cv::DetectionROI ret = (*instance)[index];
			*ocvrs_return = new cv::DetectionROI(ret);
	}

	// std::vector<cv::DetectionROI>::set(Primitive, TraitClass) generated
	// ("std::vector<cv::DetectionROI>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::DetectionROI"]), _)]),
	void std_vectorLcv_DetectionROIG_set_size_t_const_DetectionROI(std::vector<cv::DetectionROI>* instance, size_t index, const cv::DetectionROI* val) {
			(*instance)[index] = *val;
	}

}


