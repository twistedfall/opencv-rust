extern "C" {
	// std::vector<cv::dpm::DPMDetector::ObjectDetection>::new() generated
	// ("std::vector<cv::dpm::DPMDetector::ObjectDetection>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::dpm::DPMDetector::ObjectDetection>* std_vectorLcv_dpm_DPMDetector_ObjectDetectionG_new_const() {
			std::vector<cv::dpm::DPMDetector::ObjectDetection>* ret = new std::vector<cv::dpm::DPMDetector::ObjectDetection>();
			return ret;
	}

	// std::vector<cv::dpm::DPMDetector::ObjectDetection>::delete() generated
	// ("std::vector<cv::dpm::DPMDetector::ObjectDetection>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_dpm_DPMDetector_ObjectDetectionG_delete(std::vector<cv::dpm::DPMDetector::ObjectDetection>* instance) {
			delete instance;
	}

	// std::vector<cv::dpm::DPMDetector::ObjectDetection>::len() generated
	// ("std::vector<cv::dpm::DPMDetector::ObjectDetection>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_dpm_DPMDetector_ObjectDetectionG_len_const(const std::vector<cv::dpm::DPMDetector::ObjectDetection>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::dpm::DPMDetector::ObjectDetection>::isEmpty() generated
	// ("std::vector<cv::dpm::DPMDetector::ObjectDetection>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_dpm_DPMDetector_ObjectDetectionG_isEmpty_const(const std::vector<cv::dpm::DPMDetector::ObjectDetection>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::dpm::DPMDetector::ObjectDetection>::capacity() generated
	// ("std::vector<cv::dpm::DPMDetector::ObjectDetection>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_dpm_DPMDetector_ObjectDetectionG_capacity_const(const std::vector<cv::dpm::DPMDetector::ObjectDetection>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::dpm::DPMDetector::ObjectDetection>::shrinkToFit() generated
	// ("std::vector<cv::dpm::DPMDetector::ObjectDetection>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_dpm_DPMDetector_ObjectDetectionG_shrinkToFit(std::vector<cv::dpm::DPMDetector::ObjectDetection>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::dpm::DPMDetector::ObjectDetection>::reserve(Primitive) generated
	// ("std::vector<cv::dpm::DPMDetector::ObjectDetection>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_dpm_DPMDetector_ObjectDetectionG_reserve_size_t(std::vector<cv::dpm::DPMDetector::ObjectDetection>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::dpm::DPMDetector::ObjectDetection>::remove(Primitive) generated
	// ("std::vector<cv::dpm::DPMDetector::ObjectDetection>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_dpm_DPMDetector_ObjectDetectionG_remove_size_t(std::vector<cv::dpm::DPMDetector::ObjectDetection>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::dpm::DPMDetector::ObjectDetection>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::dpm::DPMDetector::ObjectDetection>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_dpm_DPMDetector_ObjectDetectionG_swap_size_t_size_t(std::vector<cv::dpm::DPMDetector::ObjectDetection>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::dpm::DPMDetector::ObjectDetection>::clear() generated
	// ("std::vector<cv::dpm::DPMDetector::ObjectDetection>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_dpm_DPMDetector_ObjectDetectionG_clear(std::vector<cv::dpm::DPMDetector::ObjectDetection>* instance) {
			instance->clear();
	}

	// std::vector<cv::dpm::DPMDetector::ObjectDetection>::push(TraitClass) generated
	// ("std::vector<cv::dpm::DPMDetector::ObjectDetection>::push", vec![(pred!(mut, ["val"], ["const cv::dpm::DPMDetector::ObjectDetection"]), _)]),
	void std_vectorLcv_dpm_DPMDetector_ObjectDetectionG_push_const_ObjectDetection(std::vector<cv::dpm::DPMDetector::ObjectDetection>* instance, const cv::dpm::DPMDetector::ObjectDetection* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::dpm::DPMDetector::ObjectDetection>::insert(Primitive, TraitClass) generated
	// ("std::vector<cv::dpm::DPMDetector::ObjectDetection>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::dpm::DPMDetector::ObjectDetection"]), _)]),
	void std_vectorLcv_dpm_DPMDetector_ObjectDetectionG_insert_size_t_const_ObjectDetection(std::vector<cv::dpm::DPMDetector::ObjectDetection>* instance, size_t index, const cv::dpm::DPMDetector::ObjectDetection* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::dpm::DPMDetector::ObjectDetection>::get(Primitive) generated
	// ("std::vector<cv::dpm::DPMDetector::ObjectDetection>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_dpm_DPMDetector_ObjectDetectionG_get_const_size_t(const std::vector<cv::dpm::DPMDetector::ObjectDetection>* instance, size_t index, cv::dpm::DPMDetector::ObjectDetection** ocvrs_return) {
			cv::dpm::DPMDetector::ObjectDetection ret = (*instance)[index];
			*ocvrs_return = new cv::dpm::DPMDetector::ObjectDetection(ret);
	}

	// std::vector<cv::dpm::DPMDetector::ObjectDetection>::set(Primitive, TraitClass) generated
	// ("std::vector<cv::dpm::DPMDetector::ObjectDetection>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::dpm::DPMDetector::ObjectDetection"]), _)]),
	void std_vectorLcv_dpm_DPMDetector_ObjectDetectionG_set_size_t_const_ObjectDetection(std::vector<cv::dpm::DPMDetector::ObjectDetection>* instance, size_t index, const cv::dpm::DPMDetector::ObjectDetection* val) {
			(*instance)[index] = *val;
	}

}


