extern "C" {
	// std::vector<std::pair<cv::Point2i, cv::Point2i>>::new() generated
	// ("std::vector<std::pair<cv::Point2i, cv::Point2i>>::new", vec![(pred!(const, [], []), _)]),
	std::vector<std::pair<cv::Point2i, cv::Point2i>>* std_vectorLstd_pairLcv_Point2i__cv_Point2iGG_new_const() {
			std::vector<std::pair<cv::Point2i, cv::Point2i>>* ret = new std::vector<std::pair<cv::Point2i, cv::Point2i>>();
			return ret;
	}

	// std::vector<std::pair<cv::Point2i, cv::Point2i>>::delete() generated
	// ("std::vector<std::pair<cv::Point2i, cv::Point2i>>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_pairLcv_Point2i__cv_Point2iGG_delete(std::vector<std::pair<cv::Point2i, cv::Point2i>>* instance) {
			delete instance;
	}

	// std::vector<std::pair<cv::Point2i, cv::Point2i>>::len() generated
	// ("std::vector<std::pair<cv::Point2i, cv::Point2i>>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_pairLcv_Point2i__cv_Point2iGG_len_const(const std::vector<std::pair<cv::Point2i, cv::Point2i>>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<std::pair<cv::Point2i, cv::Point2i>>::isEmpty() generated
	// ("std::vector<std::pair<cv::Point2i, cv::Point2i>>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLstd_pairLcv_Point2i__cv_Point2iGG_isEmpty_const(const std::vector<std::pair<cv::Point2i, cv::Point2i>>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<std::pair<cv::Point2i, cv::Point2i>>::capacity() generated
	// ("std::vector<std::pair<cv::Point2i, cv::Point2i>>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_pairLcv_Point2i__cv_Point2iGG_capacity_const(const std::vector<std::pair<cv::Point2i, cv::Point2i>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<std::pair<cv::Point2i, cv::Point2i>>::shrinkToFit() generated
	// ("std::vector<std::pair<cv::Point2i, cv::Point2i>>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_pairLcv_Point2i__cv_Point2iGG_shrinkToFit(std::vector<std::pair<cv::Point2i, cv::Point2i>>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<std::pair<cv::Point2i, cv::Point2i>>::reserve(Primitive) generated
	// ("std::vector<std::pair<cv::Point2i, cv::Point2i>>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLstd_pairLcv_Point2i__cv_Point2iGG_reserve_size_t(std::vector<std::pair<cv::Point2i, cv::Point2i>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<std::pair<cv::Point2i, cv::Point2i>>::remove(Primitive) generated
	// ("std::vector<std::pair<cv::Point2i, cv::Point2i>>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_pairLcv_Point2i__cv_Point2iGG_remove_size_t(std::vector<std::pair<cv::Point2i, cv::Point2i>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<std::pair<cv::Point2i, cv::Point2i>>::swap(Primitive, Primitive) generated
	// ("std::vector<std::pair<cv::Point2i, cv::Point2i>>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLstd_pairLcv_Point2i__cv_Point2iGG_swap_size_t_size_t(std::vector<std::pair<cv::Point2i, cv::Point2i>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<std::pair<cv::Point2i, cv::Point2i>>::clear() generated
	// ("std::vector<std::pair<cv::Point2i, cv::Point2i>>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_pairLcv_Point2i__cv_Point2iGG_clear(std::vector<std::pair<cv::Point2i, cv::Point2i>>* instance) {
			instance->clear();
	}

	// std::vector<std::pair<cv::Point2i, cv::Point2i>>::push(CppPassByVoidPtr) generated
	// ("std::vector<std::pair<cv::Point2i, cv::Point2i>>::push", vec![(pred!(mut, ["val"], ["const std::pair<cv::Point2i, cv::Point2i>"]), _)]),
	void std_vectorLstd_pairLcv_Point2i__cv_Point2iGG_push_const_pairLcv_Point2i__cv_Point2iG(std::vector<std::pair<cv::Point2i, cv::Point2i>>* instance, const std::pair<cv::Point2i, cv::Point2i>* val) {
			instance->push_back(*val);
	}

	// std::vector<std::pair<cv::Point2i, cv::Point2i>>::insert(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::pair<cv::Point2i, cv::Point2i>>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::pair<cv::Point2i, cv::Point2i>"]), _)]),
	void std_vectorLstd_pairLcv_Point2i__cv_Point2iGG_insert_size_t_const_pairLcv_Point2i__cv_Point2iG(std::vector<std::pair<cv::Point2i, cv::Point2i>>* instance, size_t index, const std::pair<cv::Point2i, cv::Point2i>* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<std::pair<cv::Point2i, cv::Point2i>>::get(Primitive) generated
	// ("std::vector<std::pair<cv::Point2i, cv::Point2i>>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_pairLcv_Point2i__cv_Point2iGG_get_const_size_t(const std::vector<std::pair<cv::Point2i, cv::Point2i>>* instance, size_t index, std::pair<cv::Point2i, cv::Point2i>** ocvrs_return) {
			std::pair<cv::Point2i, cv::Point2i> ret = (*instance)[index];
			*ocvrs_return = new std::pair<cv::Point2i, cv::Point2i>(ret);
	}

	// std::vector<std::pair<cv::Point2i, cv::Point2i>>::set(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::pair<cv::Point2i, cv::Point2i>>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::pair<cv::Point2i, cv::Point2i>"]), _)]),
	void std_vectorLstd_pairLcv_Point2i__cv_Point2iGG_set_size_t_const_pairLcv_Point2i__cv_Point2iG(std::vector<std::pair<cv::Point2i, cv::Point2i>>* instance, size_t index, const std::pair<cv::Point2i, cv::Point2i>* val) {
			(*instance)[index] = *val;
	}

}


