extern "C" {
	// std::vector<cv::detail::CameraParams>::new() generated
	// ("std::vector<cv::detail::CameraParams>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::detail::CameraParams>* std_vectorLcv_detail_CameraParamsG_new_const() {
			std::vector<cv::detail::CameraParams>* ret = new std::vector<cv::detail::CameraParams>();
			return ret;
	}

	// std::vector<cv::detail::CameraParams>::delete() generated
	// ("std::vector<cv::detail::CameraParams>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_detail_CameraParamsG_delete(std::vector<cv::detail::CameraParams>* instance) {
			delete instance;
	}

	// std::vector<cv::detail::CameraParams>::len() generated
	// ("std::vector<cv::detail::CameraParams>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_detail_CameraParamsG_len_const(const std::vector<cv::detail::CameraParams>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::detail::CameraParams>::isEmpty() generated
	// ("std::vector<cv::detail::CameraParams>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_detail_CameraParamsG_isEmpty_const(const std::vector<cv::detail::CameraParams>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::detail::CameraParams>::capacity() generated
	// ("std::vector<cv::detail::CameraParams>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_detail_CameraParamsG_capacity_const(const std::vector<cv::detail::CameraParams>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::detail::CameraParams>::shrinkToFit() generated
	// ("std::vector<cv::detail::CameraParams>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_detail_CameraParamsG_shrinkToFit(std::vector<cv::detail::CameraParams>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::detail::CameraParams>::reserve(Primitive) generated
	// ("std::vector<cv::detail::CameraParams>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_detail_CameraParamsG_reserve_size_t(std::vector<cv::detail::CameraParams>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::detail::CameraParams>::remove(Primitive) generated
	// ("std::vector<cv::detail::CameraParams>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_detail_CameraParamsG_remove_size_t(std::vector<cv::detail::CameraParams>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::detail::CameraParams>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::detail::CameraParams>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_detail_CameraParamsG_swap_size_t_size_t(std::vector<cv::detail::CameraParams>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::detail::CameraParams>::clear() generated
	// ("std::vector<cv::detail::CameraParams>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_detail_CameraParamsG_clear(std::vector<cv::detail::CameraParams>* instance) {
			instance->clear();
	}

	// std::vector<cv::detail::CameraParams>::push(TraitClass) generated
	// ("std::vector<cv::detail::CameraParams>::push", vec![(pred!(mut, ["val"], ["const cv::detail::CameraParams"]), _)]),
	void std_vectorLcv_detail_CameraParamsG_push_const_CameraParams(std::vector<cv::detail::CameraParams>* instance, const cv::detail::CameraParams* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::detail::CameraParams>::insert(Primitive, TraitClass) generated
	// ("std::vector<cv::detail::CameraParams>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::detail::CameraParams"]), _)]),
	void std_vectorLcv_detail_CameraParamsG_insert_size_t_const_CameraParams(std::vector<cv::detail::CameraParams>* instance, size_t index, const cv::detail::CameraParams* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::detail::CameraParams>::get(Primitive) generated
	// ("std::vector<cv::detail::CameraParams>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_detail_CameraParamsG_get_const_size_t(const std::vector<cv::detail::CameraParams>* instance, size_t index, cv::detail::CameraParams** ocvrs_return) {
			cv::detail::CameraParams ret = (*instance)[index];
			*ocvrs_return = new cv::detail::CameraParams(ret);
	}

	// std::vector<cv::detail::CameraParams>::set(Primitive, TraitClass) generated
	// ("std::vector<cv::detail::CameraParams>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::detail::CameraParams"]), _)]),
	void std_vectorLcv_detail_CameraParamsG_set_size_t_const_CameraParams(std::vector<cv::detail::CameraParams>* instance, size_t index, const cv::detail::CameraParams* val) {
			(*instance)[index] = *val;
	}

}


