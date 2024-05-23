extern "C" {
	// std::vector<std::vector<cv::KeyPoint>>::new() generated
	// ("std::vector<std::vector<cv::KeyPoint>>::new", vec![(pred!(const, [], []), _)]),
	std::vector<std::vector<cv::KeyPoint>>* std_vectorLstd_vectorLcv_KeyPointGG_new_const() {
			std::vector<std::vector<cv::KeyPoint>>* ret = new std::vector<std::vector<cv::KeyPoint>>();
			return ret;
	}

	// std::vector<std::vector<cv::KeyPoint>>::delete() generated
	// ("std::vector<std::vector<cv::KeyPoint>>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_KeyPointGG_delete(std::vector<std::vector<cv::KeyPoint>>* instance) {
			delete instance;
	}

	// std::vector<std::vector<cv::KeyPoint>>::len() generated
	// ("std::vector<std::vector<cv::KeyPoint>>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_vectorLcv_KeyPointGG_len_const(const std::vector<std::vector<cv::KeyPoint>>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<std::vector<cv::KeyPoint>>::isEmpty() generated
	// ("std::vector<std::vector<cv::KeyPoint>>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLstd_vectorLcv_KeyPointGG_isEmpty_const(const std::vector<std::vector<cv::KeyPoint>>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<std::vector<cv::KeyPoint>>::capacity() generated
	// ("std::vector<std::vector<cv::KeyPoint>>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_vectorLcv_KeyPointGG_capacity_const(const std::vector<std::vector<cv::KeyPoint>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<std::vector<cv::KeyPoint>>::shrinkToFit() generated
	// ("std::vector<std::vector<cv::KeyPoint>>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_KeyPointGG_shrinkToFit(std::vector<std::vector<cv::KeyPoint>>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<std::vector<cv::KeyPoint>>::reserve(Primitive) generated
	// ("std::vector<std::vector<cv::KeyPoint>>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLcv_KeyPointGG_reserve_size_t(std::vector<std::vector<cv::KeyPoint>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<std::vector<cv::KeyPoint>>::remove(Primitive) generated
	// ("std::vector<std::vector<cv::KeyPoint>>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLcv_KeyPointGG_remove_size_t(std::vector<std::vector<cv::KeyPoint>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<std::vector<cv::KeyPoint>>::swap(Primitive, Primitive) generated
	// ("std::vector<std::vector<cv::KeyPoint>>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLstd_vectorLcv_KeyPointGG_swap_size_t_size_t(std::vector<std::vector<cv::KeyPoint>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<std::vector<cv::KeyPoint>>::clear() generated
	// ("std::vector<std::vector<cv::KeyPoint>>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_KeyPointGG_clear(std::vector<std::vector<cv::KeyPoint>>* instance) {
			instance->clear();
	}

	// std::vector<std::vector<cv::KeyPoint>>::push(CppPassByVoidPtr) generated
	// ("std::vector<std::vector<cv::KeyPoint>>::push", vec![(pred!(mut, ["val"], ["const std::vector<cv::KeyPoint>"]), _)]),
	void std_vectorLstd_vectorLcv_KeyPointGG_push_const_vectorLKeyPointG(std::vector<std::vector<cv::KeyPoint>>* instance, const std::vector<cv::KeyPoint>* val) {
			instance->push_back(*val);
	}

	// std::vector<std::vector<cv::KeyPoint>>::insert(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::vector<cv::KeyPoint>>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::vector<cv::KeyPoint>"]), _)]),
	void std_vectorLstd_vectorLcv_KeyPointGG_insert_size_t_const_vectorLKeyPointG(std::vector<std::vector<cv::KeyPoint>>* instance, size_t index, const std::vector<cv::KeyPoint>* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<std::vector<cv::KeyPoint>>::get(Primitive) generated
	// ("std::vector<std::vector<cv::KeyPoint>>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLcv_KeyPointGG_get_const_size_t(const std::vector<std::vector<cv::KeyPoint>>* instance, size_t index, std::vector<cv::KeyPoint>** ocvrs_return) {
			std::vector<cv::KeyPoint> ret = (*instance)[index];
			*ocvrs_return = new std::vector<cv::KeyPoint>(ret);
	}

	// std::vector<std::vector<cv::KeyPoint>>::set(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::vector<cv::KeyPoint>>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::vector<cv::KeyPoint>"]), _)]),
	void std_vectorLstd_vectorLcv_KeyPointGG_set_size_t_const_vectorLKeyPointG(std::vector<std::vector<cv::KeyPoint>>* instance, size_t index, const std::vector<cv::KeyPoint>* val) {
			(*instance)[index] = *val;
	}

}


