extern "C" {
	// std::vector<cv::linemod::Match>::new() generated
	// ("std::vector<cv::linemod::Match>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::linemod::Match>* std_vectorLcv_linemod_MatchG_new_const() {
			std::vector<cv::linemod::Match>* ret = new std::vector<cv::linemod::Match>();
			return ret;
	}

	// std::vector<cv::linemod::Match>::delete() generated
	// ("std::vector<cv::linemod::Match>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_linemod_MatchG_delete(std::vector<cv::linemod::Match>* instance) {
			delete instance;
	}

	// std::vector<cv::linemod::Match>::len() generated
	// ("std::vector<cv::linemod::Match>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_linemod_MatchG_len_const(const std::vector<cv::linemod::Match>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::linemod::Match>::isEmpty() generated
	// ("std::vector<cv::linemod::Match>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_linemod_MatchG_isEmpty_const(const std::vector<cv::linemod::Match>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::linemod::Match>::capacity() generated
	// ("std::vector<cv::linemod::Match>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_linemod_MatchG_capacity_const(const std::vector<cv::linemod::Match>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::linemod::Match>::shrinkToFit() generated
	// ("std::vector<cv::linemod::Match>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_linemod_MatchG_shrinkToFit(std::vector<cv::linemod::Match>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::linemod::Match>::reserve(Primitive) generated
	// ("std::vector<cv::linemod::Match>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_linemod_MatchG_reserve_size_t(std::vector<cv::linemod::Match>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::linemod::Match>::remove(Primitive) generated
	// ("std::vector<cv::linemod::Match>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_linemod_MatchG_remove_size_t(std::vector<cv::linemod::Match>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::linemod::Match>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::linemod::Match>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_linemod_MatchG_swap_size_t_size_t(std::vector<cv::linemod::Match>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::linemod::Match>::clear() generated
	// ("std::vector<cv::linemod::Match>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_linemod_MatchG_clear(std::vector<cv::linemod::Match>* instance) {
			instance->clear();
	}

	// std::vector<cv::linemod::Match>::push(TraitClass) generated
	// ("std::vector<cv::linemod::Match>::push", vec![(pred!(mut, ["val"], ["const cv::linemod::Match"]), _)]),
	void std_vectorLcv_linemod_MatchG_push_const_Match(std::vector<cv::linemod::Match>* instance, const cv::linemod::Match* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::linemod::Match>::insert(Primitive, TraitClass) generated
	// ("std::vector<cv::linemod::Match>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::linemod::Match"]), _)]),
	void std_vectorLcv_linemod_MatchG_insert_size_t_const_Match(std::vector<cv::linemod::Match>* instance, size_t index, const cv::linemod::Match* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::linemod::Match>::get(Primitive) generated
	// ("std::vector<cv::linemod::Match>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_linemod_MatchG_get_const_size_t(const std::vector<cv::linemod::Match>* instance, size_t index, cv::linemod::Match** ocvrs_return) {
			cv::linemod::Match ret = (*instance)[index];
			*ocvrs_return = new cv::linemod::Match(ret);
	}

	// std::vector<cv::linemod::Match>::set(Primitive, TraitClass) generated
	// ("std::vector<cv::linemod::Match>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::linemod::Match"]), _)]),
	void std_vectorLcv_linemod_MatchG_set_size_t_const_Match(std::vector<cv::linemod::Match>* instance, size_t index, const cv::linemod::Match* val) {
			(*instance)[index] = *val;
	}

}


