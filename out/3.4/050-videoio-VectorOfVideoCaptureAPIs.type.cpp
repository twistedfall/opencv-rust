extern "C" {
	// std::vector<cv::VideoCaptureAPIs>::new() generated
	// ("std::vector<cv::VideoCaptureAPIs>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::VideoCaptureAPIs>* std_vectorLcv_VideoCaptureAPIsG_new_const() {
			std::vector<cv::VideoCaptureAPIs>* ret = new std::vector<cv::VideoCaptureAPIs>();
			return ret;
	}

	// std::vector<cv::VideoCaptureAPIs>::delete() generated
	// ("std::vector<cv::VideoCaptureAPIs>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_VideoCaptureAPIsG_delete(std::vector<cv::VideoCaptureAPIs>* instance) {
			delete instance;
	}

	// std::vector<cv::VideoCaptureAPIs>::len() generated
	// ("std::vector<cv::VideoCaptureAPIs>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_VideoCaptureAPIsG_len_const(const std::vector<cv::VideoCaptureAPIs>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::VideoCaptureAPIs>::isEmpty() generated
	// ("std::vector<cv::VideoCaptureAPIs>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_VideoCaptureAPIsG_isEmpty_const(const std::vector<cv::VideoCaptureAPIs>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::VideoCaptureAPIs>::capacity() generated
	// ("std::vector<cv::VideoCaptureAPIs>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_VideoCaptureAPIsG_capacity_const(const std::vector<cv::VideoCaptureAPIs>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::VideoCaptureAPIs>::shrinkToFit() generated
	// ("std::vector<cv::VideoCaptureAPIs>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_VideoCaptureAPIsG_shrinkToFit(std::vector<cv::VideoCaptureAPIs>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::VideoCaptureAPIs>::reserve(Primitive) generated
	// ("std::vector<cv::VideoCaptureAPIs>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_VideoCaptureAPIsG_reserve_size_t(std::vector<cv::VideoCaptureAPIs>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::VideoCaptureAPIs>::remove(Primitive) generated
	// ("std::vector<cv::VideoCaptureAPIs>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_VideoCaptureAPIsG_remove_size_t(std::vector<cv::VideoCaptureAPIs>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::VideoCaptureAPIs>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::VideoCaptureAPIs>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_VideoCaptureAPIsG_swap_size_t_size_t(std::vector<cv::VideoCaptureAPIs>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::VideoCaptureAPIs>::clear() generated
	// ("std::vector<cv::VideoCaptureAPIs>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_VideoCaptureAPIsG_clear(std::vector<cv::VideoCaptureAPIs>* instance) {
			instance->clear();
	}

	// std::vector<cv::VideoCaptureAPIs>::push(Enum) generated
	// ("std::vector<cv::VideoCaptureAPIs>::push", vec![(pred!(mut, ["val"], ["const cv::VideoCaptureAPIs"]), _)]),
	void std_vectorLcv_VideoCaptureAPIsG_push_const_VideoCaptureAPIs(std::vector<cv::VideoCaptureAPIs>* instance, const cv::VideoCaptureAPIs val) {
			instance->push_back(val);
	}

	// std::vector<cv::VideoCaptureAPIs>::insert(Primitive, Enum) generated
	// ("std::vector<cv::VideoCaptureAPIs>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::VideoCaptureAPIs"]), _)]),
	void std_vectorLcv_VideoCaptureAPIsG_insert_size_t_const_VideoCaptureAPIs(std::vector<cv::VideoCaptureAPIs>* instance, size_t index, const cv::VideoCaptureAPIs val) {
			instance->insert(instance->begin() + index, val);
	}

	// std::vector<cv::VideoCaptureAPIs>::get(Primitive) generated
	// ("std::vector<cv::VideoCaptureAPIs>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_VideoCaptureAPIsG_get_const_size_t(const std::vector<cv::VideoCaptureAPIs>* instance, size_t index, cv::VideoCaptureAPIs* ocvrs_return) {
			cv::VideoCaptureAPIs ret = (*instance)[index];
			*ocvrs_return = ret;
	}

	// std::vector<cv::VideoCaptureAPIs>::set(Primitive, Enum) generated
	// ("std::vector<cv::VideoCaptureAPIs>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::VideoCaptureAPIs"]), _)]),
	void std_vectorLcv_VideoCaptureAPIsG_set_size_t_const_VideoCaptureAPIs(std::vector<cv::VideoCaptureAPIs>* instance, size_t index, const cv::VideoCaptureAPIs val) {
			(*instance)[index] = val;
	}

	// std::vector<cv::VideoCaptureAPIs>::clone() generated
	// ("std::vector<cv::VideoCaptureAPIs>::clone", vec![(pred!(const, [], []), _)]),
	std::vector<cv::VideoCaptureAPIs>* std_vectorLcv_VideoCaptureAPIsG_clone_const(const std::vector<cv::VideoCaptureAPIs>* instance) {
			std::vector<cv::VideoCaptureAPIs> ret = std::vector<cv::VideoCaptureAPIs>(*instance);
			return new std::vector<cv::VideoCaptureAPIs>(ret);
	}

	// std::vector<cv::VideoCaptureAPIs>::data() generated
	// ("std::vector<cv::VideoCaptureAPIs>::data", vec![(pred!(const, [], []), _)]),
	const cv::VideoCaptureAPIs* std_vectorLcv_VideoCaptureAPIsG_data_const(const std::vector<cv::VideoCaptureAPIs>* instance) {
			const cv::VideoCaptureAPIs* ret = instance->data();
			return ret;
	}

	// std::vector<cv::VideoCaptureAPIs>::dataMut() generated
	// ("std::vector<cv::VideoCaptureAPIs>::dataMut", vec![(pred!(mut, [], []), _)]),
	cv::VideoCaptureAPIs* std_vectorLcv_VideoCaptureAPIsG_dataMut(std::vector<cv::VideoCaptureAPIs>* instance) {
			cv::VideoCaptureAPIs* ret = instance->data();
			return ret;
	}

	// cv::fromSlice(Enum, Primitive) generated
	// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const cv::VideoCaptureAPIs*", "size_t"]), _)]),
	std::vector<cv::VideoCaptureAPIs>* cv_fromSlice_const_const_VideoCaptureAPIsX_size_t(const cv::VideoCaptureAPIs* data, size_t len) {
			return new std::vector<cv::VideoCaptureAPIs>(data, data + len);
	}

}


