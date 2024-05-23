extern "C" {
	// std::vector<std::vector<cv::text::ERStat>>::new() generated
	// ("std::vector<std::vector<cv::text::ERStat>>::new", vec![(pred!(const, [], []), _)]),
	std::vector<std::vector<cv::text::ERStat>>* std_vectorLstd_vectorLcv_text_ERStatGG_new_const() {
			std::vector<std::vector<cv::text::ERStat>>* ret = new std::vector<std::vector<cv::text::ERStat>>();
			return ret;
	}

	// std::vector<std::vector<cv::text::ERStat>>::delete() generated
	// ("std::vector<std::vector<cv::text::ERStat>>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_text_ERStatGG_delete(std::vector<std::vector<cv::text::ERStat>>* instance) {
			delete instance;
	}

	// std::vector<std::vector<cv::text::ERStat>>::len() generated
	// ("std::vector<std::vector<cv::text::ERStat>>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_vectorLcv_text_ERStatGG_len_const(const std::vector<std::vector<cv::text::ERStat>>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<std::vector<cv::text::ERStat>>::isEmpty() generated
	// ("std::vector<std::vector<cv::text::ERStat>>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLstd_vectorLcv_text_ERStatGG_isEmpty_const(const std::vector<std::vector<cv::text::ERStat>>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<std::vector<cv::text::ERStat>>::capacity() generated
	// ("std::vector<std::vector<cv::text::ERStat>>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_vectorLcv_text_ERStatGG_capacity_const(const std::vector<std::vector<cv::text::ERStat>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<std::vector<cv::text::ERStat>>::shrinkToFit() generated
	// ("std::vector<std::vector<cv::text::ERStat>>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_text_ERStatGG_shrinkToFit(std::vector<std::vector<cv::text::ERStat>>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<std::vector<cv::text::ERStat>>::reserve(Primitive) generated
	// ("std::vector<std::vector<cv::text::ERStat>>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLcv_text_ERStatGG_reserve_size_t(std::vector<std::vector<cv::text::ERStat>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<std::vector<cv::text::ERStat>>::remove(Primitive) generated
	// ("std::vector<std::vector<cv::text::ERStat>>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLcv_text_ERStatGG_remove_size_t(std::vector<std::vector<cv::text::ERStat>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<std::vector<cv::text::ERStat>>::swap(Primitive, Primitive) generated
	// ("std::vector<std::vector<cv::text::ERStat>>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLstd_vectorLcv_text_ERStatGG_swap_size_t_size_t(std::vector<std::vector<cv::text::ERStat>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<std::vector<cv::text::ERStat>>::clear() generated
	// ("std::vector<std::vector<cv::text::ERStat>>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_text_ERStatGG_clear(std::vector<std::vector<cv::text::ERStat>>* instance) {
			instance->clear();
	}

	// std::vector<std::vector<cv::text::ERStat>>::push(CppPassByVoidPtr) generated
	// ("std::vector<std::vector<cv::text::ERStat>>::push", vec![(pred!(mut, ["val"], ["const std::vector<cv::text::ERStat>"]), _)]),
	void std_vectorLstd_vectorLcv_text_ERStatGG_push_const_vectorLERStatG(std::vector<std::vector<cv::text::ERStat>>* instance, const std::vector<cv::text::ERStat>* val) {
			instance->push_back(*val);
	}

	// std::vector<std::vector<cv::text::ERStat>>::insert(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::vector<cv::text::ERStat>>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::vector<cv::text::ERStat>"]), _)]),
	void std_vectorLstd_vectorLcv_text_ERStatGG_insert_size_t_const_vectorLERStatG(std::vector<std::vector<cv::text::ERStat>>* instance, size_t index, const std::vector<cv::text::ERStat>* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<std::vector<cv::text::ERStat>>::get(Primitive) generated
	// ("std::vector<std::vector<cv::text::ERStat>>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLcv_text_ERStatGG_get_const_size_t(const std::vector<std::vector<cv::text::ERStat>>* instance, size_t index, std::vector<cv::text::ERStat>** ocvrs_return) {
			std::vector<cv::text::ERStat> ret = (*instance)[index];
			*ocvrs_return = new std::vector<cv::text::ERStat>(ret);
	}

	// std::vector<std::vector<cv::text::ERStat>>::set(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::vector<cv::text::ERStat>>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::vector<cv::text::ERStat>"]), _)]),
	void std_vectorLstd_vectorLcv_text_ERStatGG_set_size_t_const_vectorLERStatG(std::vector<std::vector<cv::text::ERStat>>* instance, size_t index, const std::vector<cv::text::ERStat>* val) {
			(*instance)[index] = *val;
	}

}


