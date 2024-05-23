extern "C" {
	// std::vector<cv::detail::ImageFeatures>::new() generated
	// ("std::vector<cv::detail::ImageFeatures>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::detail::ImageFeatures>* std_vectorLcv_detail_ImageFeaturesG_new_const() {
			std::vector<cv::detail::ImageFeatures>* ret = new std::vector<cv::detail::ImageFeatures>();
			return ret;
	}

	// std::vector<cv::detail::ImageFeatures>::delete() generated
	// ("std::vector<cv::detail::ImageFeatures>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_detail_ImageFeaturesG_delete(std::vector<cv::detail::ImageFeatures>* instance) {
			delete instance;
	}

	// std::vector<cv::detail::ImageFeatures>::len() generated
	// ("std::vector<cv::detail::ImageFeatures>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_detail_ImageFeaturesG_len_const(const std::vector<cv::detail::ImageFeatures>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::detail::ImageFeatures>::isEmpty() generated
	// ("std::vector<cv::detail::ImageFeatures>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_detail_ImageFeaturesG_isEmpty_const(const std::vector<cv::detail::ImageFeatures>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::detail::ImageFeatures>::capacity() generated
	// ("std::vector<cv::detail::ImageFeatures>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_detail_ImageFeaturesG_capacity_const(const std::vector<cv::detail::ImageFeatures>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::detail::ImageFeatures>::shrinkToFit() generated
	// ("std::vector<cv::detail::ImageFeatures>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_detail_ImageFeaturesG_shrinkToFit(std::vector<cv::detail::ImageFeatures>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::detail::ImageFeatures>::reserve(Primitive) generated
	// ("std::vector<cv::detail::ImageFeatures>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_detail_ImageFeaturesG_reserve_size_t(std::vector<cv::detail::ImageFeatures>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::detail::ImageFeatures>::remove(Primitive) generated
	// ("std::vector<cv::detail::ImageFeatures>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_detail_ImageFeaturesG_remove_size_t(std::vector<cv::detail::ImageFeatures>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::detail::ImageFeatures>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::detail::ImageFeatures>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_detail_ImageFeaturesG_swap_size_t_size_t(std::vector<cv::detail::ImageFeatures>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::detail::ImageFeatures>::clear() generated
	// ("std::vector<cv::detail::ImageFeatures>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_detail_ImageFeaturesG_clear(std::vector<cv::detail::ImageFeatures>* instance) {
			instance->clear();
	}

	// std::vector<cv::detail::ImageFeatures>::push(TraitClass) generated
	// ("std::vector<cv::detail::ImageFeatures>::push", vec![(pred!(mut, ["val"], ["const cv::detail::ImageFeatures"]), _)]),
	void std_vectorLcv_detail_ImageFeaturesG_push_const_ImageFeatures(std::vector<cv::detail::ImageFeatures>* instance, const cv::detail::ImageFeatures* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::detail::ImageFeatures>::insert(Primitive, TraitClass) generated
	// ("std::vector<cv::detail::ImageFeatures>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::detail::ImageFeatures"]), _)]),
	void std_vectorLcv_detail_ImageFeaturesG_insert_size_t_const_ImageFeatures(std::vector<cv::detail::ImageFeatures>* instance, size_t index, const cv::detail::ImageFeatures* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::detail::ImageFeatures>::get(Primitive) generated
	// ("std::vector<cv::detail::ImageFeatures>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_detail_ImageFeaturesG_get_const_size_t(const std::vector<cv::detail::ImageFeatures>* instance, size_t index, cv::detail::ImageFeatures** ocvrs_return) {
			cv::detail::ImageFeatures ret = (*instance)[index];
			*ocvrs_return = new cv::detail::ImageFeatures(ret);
	}

	// std::vector<cv::detail::ImageFeatures>::set(Primitive, TraitClass) generated
	// ("std::vector<cv::detail::ImageFeatures>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::detail::ImageFeatures"]), _)]),
	void std_vectorLcv_detail_ImageFeaturesG_set_size_t_const_ImageFeatures(std::vector<cv::detail::ImageFeatures>* instance, size_t index, const cv::detail::ImageFeatures* val) {
			(*instance)[index] = *val;
	}

}


