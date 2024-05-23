extern "C" {
	// std::vector<cv::text::ERStat>::new() generated
	// ("std::vector<cv::text::ERStat>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::text::ERStat>* std_vectorLcv_text_ERStatG_new_const() {
			std::vector<cv::text::ERStat>* ret = new std::vector<cv::text::ERStat>();
			return ret;
	}

	// std::vector<cv::text::ERStat>::delete() generated
	// ("std::vector<cv::text::ERStat>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_text_ERStatG_delete(std::vector<cv::text::ERStat>* instance) {
			delete instance;
	}

	// std::vector<cv::text::ERStat>::len() generated
	// ("std::vector<cv::text::ERStat>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_text_ERStatG_len_const(const std::vector<cv::text::ERStat>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::text::ERStat>::isEmpty() generated
	// ("std::vector<cv::text::ERStat>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_text_ERStatG_isEmpty_const(const std::vector<cv::text::ERStat>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::text::ERStat>::capacity() generated
	// ("std::vector<cv::text::ERStat>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_text_ERStatG_capacity_const(const std::vector<cv::text::ERStat>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::text::ERStat>::shrinkToFit() generated
	// ("std::vector<cv::text::ERStat>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_text_ERStatG_shrinkToFit(std::vector<cv::text::ERStat>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::text::ERStat>::reserve(Primitive) generated
	// ("std::vector<cv::text::ERStat>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_text_ERStatG_reserve_size_t(std::vector<cv::text::ERStat>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::text::ERStat>::remove(Primitive) generated
	// ("std::vector<cv::text::ERStat>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_text_ERStatG_remove_size_t(std::vector<cv::text::ERStat>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::text::ERStat>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::text::ERStat>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_text_ERStatG_swap_size_t_size_t(std::vector<cv::text::ERStat>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::text::ERStat>::clear() generated
	// ("std::vector<cv::text::ERStat>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_text_ERStatG_clear(std::vector<cv::text::ERStat>* instance) {
			instance->clear();
	}

	// std::vector<cv::text::ERStat>::push(TraitClass) generated
	// ("std::vector<cv::text::ERStat>::push", vec![(pred!(mut, ["val"], ["const cv::text::ERStat"]), _)]),
	void std_vectorLcv_text_ERStatG_push_const_ERStat(std::vector<cv::text::ERStat>* instance, const cv::text::ERStat* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::text::ERStat>::insert(Primitive, TraitClass) generated
	// ("std::vector<cv::text::ERStat>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::text::ERStat"]), _)]),
	void std_vectorLcv_text_ERStatG_insert_size_t_const_ERStat(std::vector<cv::text::ERStat>* instance, size_t index, const cv::text::ERStat* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::text::ERStat>::get(Primitive) generated
	// ("std::vector<cv::text::ERStat>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_text_ERStatG_get_const_size_t(const std::vector<cv::text::ERStat>* instance, size_t index, cv::text::ERStat** ocvrs_return) {
			cv::text::ERStat ret = (*instance)[index];
			*ocvrs_return = new cv::text::ERStat(ret);
	}

	// std::vector<cv::text::ERStat>::set(Primitive, TraitClass) generated
	// ("std::vector<cv::text::ERStat>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::text::ERStat"]), _)]),
	void std_vectorLcv_text_ERStatG_set_size_t_const_ERStat(std::vector<cv::text::ERStat>* instance, size_t index, const cv::text::ERStat* val) {
			(*instance)[index] = *val;
	}

}


