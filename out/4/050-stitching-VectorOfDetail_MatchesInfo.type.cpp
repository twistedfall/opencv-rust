extern "C" {
	// std::vector<cv::detail::MatchesInfo>::new() generated
	// ("std::vector<cv::detail::MatchesInfo>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::detail::MatchesInfo>* std_vectorLcv_detail_MatchesInfoG_new_const() {
			std::vector<cv::detail::MatchesInfo>* ret = new std::vector<cv::detail::MatchesInfo>();
			return ret;
	}

	// std::vector<cv::detail::MatchesInfo>::delete() generated
	// ("std::vector<cv::detail::MatchesInfo>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_detail_MatchesInfoG_delete(std::vector<cv::detail::MatchesInfo>* instance) {
			delete instance;
	}

	// std::vector<cv::detail::MatchesInfo>::len() generated
	// ("std::vector<cv::detail::MatchesInfo>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_detail_MatchesInfoG_len_const(const std::vector<cv::detail::MatchesInfo>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::detail::MatchesInfo>::isEmpty() generated
	// ("std::vector<cv::detail::MatchesInfo>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_detail_MatchesInfoG_isEmpty_const(const std::vector<cv::detail::MatchesInfo>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::detail::MatchesInfo>::capacity() generated
	// ("std::vector<cv::detail::MatchesInfo>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_detail_MatchesInfoG_capacity_const(const std::vector<cv::detail::MatchesInfo>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::detail::MatchesInfo>::shrinkToFit() generated
	// ("std::vector<cv::detail::MatchesInfo>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_detail_MatchesInfoG_shrinkToFit(std::vector<cv::detail::MatchesInfo>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::detail::MatchesInfo>::reserve(Primitive) generated
	// ("std::vector<cv::detail::MatchesInfo>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_detail_MatchesInfoG_reserve_size_t(std::vector<cv::detail::MatchesInfo>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::detail::MatchesInfo>::remove(Primitive) generated
	// ("std::vector<cv::detail::MatchesInfo>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_detail_MatchesInfoG_remove_size_t(std::vector<cv::detail::MatchesInfo>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::detail::MatchesInfo>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::detail::MatchesInfo>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_detail_MatchesInfoG_swap_size_t_size_t(std::vector<cv::detail::MatchesInfo>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::detail::MatchesInfo>::clear() generated
	// ("std::vector<cv::detail::MatchesInfo>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_detail_MatchesInfoG_clear(std::vector<cv::detail::MatchesInfo>* instance) {
			instance->clear();
	}

	// std::vector<cv::detail::MatchesInfo>::push(TraitClass) generated
	// ("std::vector<cv::detail::MatchesInfo>::push", vec![(pred!(mut, ["val"], ["const cv::detail::MatchesInfo"]), _)]),
	void std_vectorLcv_detail_MatchesInfoG_push_const_MatchesInfo(std::vector<cv::detail::MatchesInfo>* instance, const cv::detail::MatchesInfo* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::detail::MatchesInfo>::insert(Primitive, TraitClass) generated
	// ("std::vector<cv::detail::MatchesInfo>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::detail::MatchesInfo"]), _)]),
	void std_vectorLcv_detail_MatchesInfoG_insert_size_t_const_MatchesInfo(std::vector<cv::detail::MatchesInfo>* instance, size_t index, const cv::detail::MatchesInfo* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::detail::MatchesInfo>::get(Primitive) generated
	// ("std::vector<cv::detail::MatchesInfo>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_detail_MatchesInfoG_get_const_size_t(const std::vector<cv::detail::MatchesInfo>* instance, size_t index, cv::detail::MatchesInfo** ocvrs_return) {
			cv::detail::MatchesInfo ret = (*instance)[index];
			*ocvrs_return = new cv::detail::MatchesInfo(ret);
	}

	// std::vector<cv::detail::MatchesInfo>::set(Primitive, TraitClass) generated
	// ("std::vector<cv::detail::MatchesInfo>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::detail::MatchesInfo"]), _)]),
	void std_vectorLcv_detail_MatchesInfoG_set_size_t_const_MatchesInfo(std::vector<cv::detail::MatchesInfo>* instance, size_t index, const cv::detail::MatchesInfo* val) {
			(*instance)[index] = *val;
	}

}


