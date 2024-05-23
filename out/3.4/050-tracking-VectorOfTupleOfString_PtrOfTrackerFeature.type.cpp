extern "C" {
	// std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>>::new() generated
	// ("std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>>::new", vec![(pred!(const, [], []), _)]),
	std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>>* std_vectorLstd_pairLcv_String__cv_PtrLcv_TrackerFeatureGGG_new_const() {
			std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>>* ret = new std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>>();
			return ret;
	}

	// std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>>::delete() generated
	// ("std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_pairLcv_String__cv_PtrLcv_TrackerFeatureGGG_delete(std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>>* instance) {
			delete instance;
	}

	// std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>>::len() generated
	// ("std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_pairLcv_String__cv_PtrLcv_TrackerFeatureGGG_len_const(const std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>>::isEmpty() generated
	// ("std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLstd_pairLcv_String__cv_PtrLcv_TrackerFeatureGGG_isEmpty_const(const std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>>::capacity() generated
	// ("std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_pairLcv_String__cv_PtrLcv_TrackerFeatureGGG_capacity_const(const std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>>::shrinkToFit() generated
	// ("std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_pairLcv_String__cv_PtrLcv_TrackerFeatureGGG_shrinkToFit(std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>>::reserve(Primitive) generated
	// ("std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLstd_pairLcv_String__cv_PtrLcv_TrackerFeatureGGG_reserve_size_t(std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>>::remove(Primitive) generated
	// ("std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_pairLcv_String__cv_PtrLcv_TrackerFeatureGGG_remove_size_t(std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>>::swap(Primitive, Primitive) generated
	// ("std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLstd_pairLcv_String__cv_PtrLcv_TrackerFeatureGGG_swap_size_t_size_t(std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>>::clear() generated
	// ("std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_pairLcv_String__cv_PtrLcv_TrackerFeatureGGG_clear(std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>>* instance) {
			instance->clear();
	}

	// std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>>::push(CppPassByVoidPtr) generated
	// ("std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>>::push", vec![(pred!(mut, ["val"], ["const std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>"]), _)]),
	void std_vectorLstd_pairLcv_String__cv_PtrLcv_TrackerFeatureGGG_push_const_pairLcv_String__cv_PtrLcv_TrackerFeatureGG(std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>>* instance, const std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>* val) {
			instance->push_back(*val);
	}

	// std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>>::insert(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>"]), _)]),
	void std_vectorLstd_pairLcv_String__cv_PtrLcv_TrackerFeatureGGG_insert_size_t_const_pairLcv_String__cv_PtrLcv_TrackerFeatureGG(std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>>* instance, size_t index, const std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>>::get(Primitive) generated
	// ("std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_pairLcv_String__cv_PtrLcv_TrackerFeatureGGG_get_const_size_t(const std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>>* instance, size_t index, std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>** ocvrs_return) {
			std::pair<cv::String, cv::Ptr<cv::TrackerFeature>> ret = (*instance)[index];
			*ocvrs_return = new std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>(ret);
	}

	// std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>>::set(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>"]), _)]),
	void std_vectorLstd_pairLcv_String__cv_PtrLcv_TrackerFeatureGGG_set_size_t_const_pairLcv_String__cv_PtrLcv_TrackerFeatureGG(std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>>* instance, size_t index, const std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>* val) {
			(*instance)[index] = *val;
	}

}


