extern "C" {
	// std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>>::new() generated
	// ("std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>>::new", vec![(pred!(const, [], []), _)]),
	std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>>* std_vectorLstd_pairLcv_String__cv_PtrLcv_TrackerSamplerAlgorithmGGG_new_const() {
			std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>>* ret = new std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>>();
			return ret;
	}

	// std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>>::delete() generated
	// ("std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_pairLcv_String__cv_PtrLcv_TrackerSamplerAlgorithmGGG_delete(std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>>* instance) {
			delete instance;
	}

	// std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>>::len() generated
	// ("std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_pairLcv_String__cv_PtrLcv_TrackerSamplerAlgorithmGGG_len_const(const std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>>::isEmpty() generated
	// ("std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLstd_pairLcv_String__cv_PtrLcv_TrackerSamplerAlgorithmGGG_isEmpty_const(const std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>>::capacity() generated
	// ("std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_pairLcv_String__cv_PtrLcv_TrackerSamplerAlgorithmGGG_capacity_const(const std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>>::shrinkToFit() generated
	// ("std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_pairLcv_String__cv_PtrLcv_TrackerSamplerAlgorithmGGG_shrinkToFit(std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>>::reserve(Primitive) generated
	// ("std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLstd_pairLcv_String__cv_PtrLcv_TrackerSamplerAlgorithmGGG_reserve_size_t(std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>>::remove(Primitive) generated
	// ("std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_pairLcv_String__cv_PtrLcv_TrackerSamplerAlgorithmGGG_remove_size_t(std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>>::swap(Primitive, Primitive) generated
	// ("std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLstd_pairLcv_String__cv_PtrLcv_TrackerSamplerAlgorithmGGG_swap_size_t_size_t(std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>>::clear() generated
	// ("std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_pairLcv_String__cv_PtrLcv_TrackerSamplerAlgorithmGGG_clear(std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>>* instance) {
			instance->clear();
	}

	// std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>>::push(CppPassByVoidPtr) generated
	// ("std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>>::push", vec![(pred!(mut, ["val"], ["const std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>"]), _)]),
	void std_vectorLstd_pairLcv_String__cv_PtrLcv_TrackerSamplerAlgorithmGGG_push_const_pairLcv_String__cv_PtrLcv_TrackerSamplerAlgorithmGG(std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>>* instance, const std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>* val) {
			instance->push_back(*val);
	}

	// std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>>::insert(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>"]), _)]),
	void std_vectorLstd_pairLcv_String__cv_PtrLcv_TrackerSamplerAlgorithmGGG_insert_size_t_const_pairLcv_String__cv_PtrLcv_TrackerSamplerAlgorithmGG(std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>>* instance, size_t index, const std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>>::get(Primitive) generated
	// ("std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_pairLcv_String__cv_PtrLcv_TrackerSamplerAlgorithmGGG_get_const_size_t(const std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>>* instance, size_t index, std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>** ocvrs_return) {
			std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>> ret = (*instance)[index];
			*ocvrs_return = new std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>(ret);
	}

	// std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>>::set(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>"]), _)]),
	void std_vectorLstd_pairLcv_String__cv_PtrLcv_TrackerSamplerAlgorithmGGG_set_size_t_const_pairLcv_String__cv_PtrLcv_TrackerSamplerAlgorithmGG(std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>>* instance, size_t index, const std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>* val) {
			(*instance)[index] = *val;
	}

}


