extern "C" {
	// std::vector<cv::xfeatures2d::Elliptic_KeyPoint>::new() generated
	// ("std::vector<cv::xfeatures2d::Elliptic_KeyPoint>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* std_vectorLcv_xfeatures2d_Elliptic_KeyPointG_new_const() {
			std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* ret = new std::vector<cv::xfeatures2d::Elliptic_KeyPoint>();
			return ret;
	}

	// std::vector<cv::xfeatures2d::Elliptic_KeyPoint>::delete() generated
	// ("std::vector<cv::xfeatures2d::Elliptic_KeyPoint>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_xfeatures2d_Elliptic_KeyPointG_delete(std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* instance) {
			delete instance;
	}

	// std::vector<cv::xfeatures2d::Elliptic_KeyPoint>::len() generated
	// ("std::vector<cv::xfeatures2d::Elliptic_KeyPoint>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_xfeatures2d_Elliptic_KeyPointG_len_const(const std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::xfeatures2d::Elliptic_KeyPoint>::isEmpty() generated
	// ("std::vector<cv::xfeatures2d::Elliptic_KeyPoint>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_xfeatures2d_Elliptic_KeyPointG_isEmpty_const(const std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::xfeatures2d::Elliptic_KeyPoint>::capacity() generated
	// ("std::vector<cv::xfeatures2d::Elliptic_KeyPoint>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_xfeatures2d_Elliptic_KeyPointG_capacity_const(const std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::xfeatures2d::Elliptic_KeyPoint>::shrinkToFit() generated
	// ("std::vector<cv::xfeatures2d::Elliptic_KeyPoint>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_xfeatures2d_Elliptic_KeyPointG_shrinkToFit(std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::xfeatures2d::Elliptic_KeyPoint>::reserve(Primitive) generated
	// ("std::vector<cv::xfeatures2d::Elliptic_KeyPoint>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_xfeatures2d_Elliptic_KeyPointG_reserve_size_t(std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::xfeatures2d::Elliptic_KeyPoint>::remove(Primitive) generated
	// ("std::vector<cv::xfeatures2d::Elliptic_KeyPoint>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_xfeatures2d_Elliptic_KeyPointG_remove_size_t(std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::xfeatures2d::Elliptic_KeyPoint>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::xfeatures2d::Elliptic_KeyPoint>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_xfeatures2d_Elliptic_KeyPointG_swap_size_t_size_t(std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::xfeatures2d::Elliptic_KeyPoint>::clear() generated
	// ("std::vector<cv::xfeatures2d::Elliptic_KeyPoint>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_xfeatures2d_Elliptic_KeyPointG_clear(std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* instance) {
			instance->clear();
	}

	// std::vector<cv::xfeatures2d::Elliptic_KeyPoint>::push(TraitClass) generated
	// ("std::vector<cv::xfeatures2d::Elliptic_KeyPoint>::push", vec![(pred!(mut, ["val"], ["const cv::xfeatures2d::Elliptic_KeyPoint"]), _)]),
	void std_vectorLcv_xfeatures2d_Elliptic_KeyPointG_push_const_Elliptic_KeyPoint(std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* instance, const cv::xfeatures2d::Elliptic_KeyPoint* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::xfeatures2d::Elliptic_KeyPoint>::insert(Primitive, TraitClass) generated
	// ("std::vector<cv::xfeatures2d::Elliptic_KeyPoint>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::xfeatures2d::Elliptic_KeyPoint"]), _)]),
	void std_vectorLcv_xfeatures2d_Elliptic_KeyPointG_insert_size_t_const_Elliptic_KeyPoint(std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* instance, size_t index, const cv::xfeatures2d::Elliptic_KeyPoint* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::xfeatures2d::Elliptic_KeyPoint>::get(Primitive) generated
	// ("std::vector<cv::xfeatures2d::Elliptic_KeyPoint>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_xfeatures2d_Elliptic_KeyPointG_get_const_size_t(const std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* instance, size_t index, cv::xfeatures2d::Elliptic_KeyPoint** ocvrs_return) {
			cv::xfeatures2d::Elliptic_KeyPoint ret = (*instance)[index];
			*ocvrs_return = new cv::xfeatures2d::Elliptic_KeyPoint(ret);
	}

	// std::vector<cv::xfeatures2d::Elliptic_KeyPoint>::set(Primitive, TraitClass) generated
	// ("std::vector<cv::xfeatures2d::Elliptic_KeyPoint>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::xfeatures2d::Elliptic_KeyPoint"]), _)]),
	void std_vectorLcv_xfeatures2d_Elliptic_KeyPointG_set_size_t_const_Elliptic_KeyPoint(std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* instance, size_t index, const cv::xfeatures2d::Elliptic_KeyPoint* val) {
			(*instance)[index] = *val;
	}

}


