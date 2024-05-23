extern "C" {
	// std::vector<std::vector<cv::DMatch>>::new() generated
	// ("std::vector<std::vector<cv::DMatch>>::new", vec![(pred!(const, [], []), _)]),
	std::vector<std::vector<cv::DMatch>>* std_vectorLstd_vectorLcv_DMatchGG_new_const() {
			std::vector<std::vector<cv::DMatch>>* ret = new std::vector<std::vector<cv::DMatch>>();
			return ret;
	}

	// std::vector<std::vector<cv::DMatch>>::delete() generated
	// ("std::vector<std::vector<cv::DMatch>>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_DMatchGG_delete(std::vector<std::vector<cv::DMatch>>* instance) {
			delete instance;
	}

	// std::vector<std::vector<cv::DMatch>>::len() generated
	// ("std::vector<std::vector<cv::DMatch>>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_vectorLcv_DMatchGG_len_const(const std::vector<std::vector<cv::DMatch>>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<std::vector<cv::DMatch>>::isEmpty() generated
	// ("std::vector<std::vector<cv::DMatch>>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLstd_vectorLcv_DMatchGG_isEmpty_const(const std::vector<std::vector<cv::DMatch>>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<std::vector<cv::DMatch>>::capacity() generated
	// ("std::vector<std::vector<cv::DMatch>>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_vectorLcv_DMatchGG_capacity_const(const std::vector<std::vector<cv::DMatch>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<std::vector<cv::DMatch>>::shrinkToFit() generated
	// ("std::vector<std::vector<cv::DMatch>>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_DMatchGG_shrinkToFit(std::vector<std::vector<cv::DMatch>>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<std::vector<cv::DMatch>>::reserve(Primitive) generated
	// ("std::vector<std::vector<cv::DMatch>>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLcv_DMatchGG_reserve_size_t(std::vector<std::vector<cv::DMatch>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<std::vector<cv::DMatch>>::remove(Primitive) generated
	// ("std::vector<std::vector<cv::DMatch>>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLcv_DMatchGG_remove_size_t(std::vector<std::vector<cv::DMatch>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<std::vector<cv::DMatch>>::swap(Primitive, Primitive) generated
	// ("std::vector<std::vector<cv::DMatch>>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLstd_vectorLcv_DMatchGG_swap_size_t_size_t(std::vector<std::vector<cv::DMatch>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<std::vector<cv::DMatch>>::clear() generated
	// ("std::vector<std::vector<cv::DMatch>>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_DMatchGG_clear(std::vector<std::vector<cv::DMatch>>* instance) {
			instance->clear();
	}

	// std::vector<std::vector<cv::DMatch>>::push(CppPassByVoidPtr) generated
	// ("std::vector<std::vector<cv::DMatch>>::push", vec![(pred!(mut, ["val"], ["const std::vector<cv::DMatch>"]), _)]),
	void std_vectorLstd_vectorLcv_DMatchGG_push_const_vectorLDMatchG(std::vector<std::vector<cv::DMatch>>* instance, const std::vector<cv::DMatch>* val) {
			instance->push_back(*val);
	}

	// std::vector<std::vector<cv::DMatch>>::insert(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::vector<cv::DMatch>>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::vector<cv::DMatch>"]), _)]),
	void std_vectorLstd_vectorLcv_DMatchGG_insert_size_t_const_vectorLDMatchG(std::vector<std::vector<cv::DMatch>>* instance, size_t index, const std::vector<cv::DMatch>* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<std::vector<cv::DMatch>>::get(Primitive) generated
	// ("std::vector<std::vector<cv::DMatch>>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLcv_DMatchGG_get_const_size_t(const std::vector<std::vector<cv::DMatch>>* instance, size_t index, std::vector<cv::DMatch>** ocvrs_return) {
			std::vector<cv::DMatch> ret = (*instance)[index];
			*ocvrs_return = new std::vector<cv::DMatch>(ret);
	}

	// std::vector<std::vector<cv::DMatch>>::set(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::vector<cv::DMatch>>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::vector<cv::DMatch>"]), _)]),
	void std_vectorLstd_vectorLcv_DMatchGG_set_size_t_const_vectorLDMatchG(std::vector<std::vector<cv::DMatch>>* instance, size_t index, const std::vector<cv::DMatch>* val) {
			(*instance)[index] = *val;
	}

}


