extern "C" {
	// std::vector<cv::VideoCapture>::new() generated
	// ("std::vector<cv::VideoCapture>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::VideoCapture>* std_vectorLcv_VideoCaptureG_new_const() {
			std::vector<cv::VideoCapture>* ret = new std::vector<cv::VideoCapture>();
			return ret;
	}

	// std::vector<cv::VideoCapture>::delete() generated
	// ("std::vector<cv::VideoCapture>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_VideoCaptureG_delete(std::vector<cv::VideoCapture>* instance) {
			delete instance;
	}

	// std::vector<cv::VideoCapture>::len() generated
	// ("std::vector<cv::VideoCapture>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_VideoCaptureG_len_const(const std::vector<cv::VideoCapture>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::VideoCapture>::isEmpty() generated
	// ("std::vector<cv::VideoCapture>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_VideoCaptureG_isEmpty_const(const std::vector<cv::VideoCapture>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::VideoCapture>::capacity() generated
	// ("std::vector<cv::VideoCapture>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_VideoCaptureG_capacity_const(const std::vector<cv::VideoCapture>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::VideoCapture>::shrinkToFit() generated
	// ("std::vector<cv::VideoCapture>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_VideoCaptureG_shrinkToFit(std::vector<cv::VideoCapture>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::VideoCapture>::reserve(Primitive) generated
	// ("std::vector<cv::VideoCapture>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_VideoCaptureG_reserve_size_t(std::vector<cv::VideoCapture>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::VideoCapture>::remove(Primitive) generated
	// ("std::vector<cv::VideoCapture>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_VideoCaptureG_remove_size_t(std::vector<cv::VideoCapture>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::VideoCapture>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::VideoCapture>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_VideoCaptureG_swap_size_t_size_t(std::vector<cv::VideoCapture>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::VideoCapture>::clear() generated
	// ("std::vector<cv::VideoCapture>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_VideoCaptureG_clear(std::vector<cv::VideoCapture>* instance) {
			instance->clear();
	}

	// std::vector<cv::VideoCapture>::push(TraitClass) generated
	// ("std::vector<cv::VideoCapture>::push", vec![(pred!(mut, ["val"], ["const cv::VideoCapture"]), _)]),
	void std_vectorLcv_VideoCaptureG_push_const_VideoCapture(std::vector<cv::VideoCapture>* instance, const cv::VideoCapture* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::VideoCapture>::insert(Primitive, TraitClass) generated
	// ("std::vector<cv::VideoCapture>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::VideoCapture"]), _)]),
	void std_vectorLcv_VideoCaptureG_insert_size_t_const_VideoCapture(std::vector<cv::VideoCapture>* instance, size_t index, const cv::VideoCapture* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::VideoCapture>::get(Primitive) generated
	// ("std::vector<cv::VideoCapture>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_VideoCaptureG_get_const_size_t(const std::vector<cv::VideoCapture>* instance, size_t index, cv::VideoCapture** ocvrs_return) {
			cv::VideoCapture ret = (*instance)[index];
			*ocvrs_return = new cv::VideoCapture(ret);
	}

	// std::vector<cv::VideoCapture>::set(Primitive, TraitClass) generated
	// ("std::vector<cv::VideoCapture>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::VideoCapture"]), _)]),
	void std_vectorLcv_VideoCaptureG_set_size_t_const_VideoCapture(std::vector<cv::VideoCapture>* instance, size_t index, const cv::VideoCapture* val) {
			(*instance)[index] = *val;
	}

}


