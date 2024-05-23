extern "C" {
	// std::vector<std::pair<cv::dnn::Backend, cv::dnn::Target>>::new() generated
	// ("std::vector<std::pair<cv::dnn::Backend, cv::dnn::Target>>::new", vec![(pred!(const, [], []), _)]),
	std::vector<std::pair<cv::dnn::Backend, cv::dnn::Target>>* std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_new_const() {
			std::vector<std::pair<cv::dnn::Backend, cv::dnn::Target>>* ret = new std::vector<std::pair<cv::dnn::Backend, cv::dnn::Target>>();
			return ret;
	}

	// std::vector<std::pair<cv::dnn::Backend, cv::dnn::Target>>::delete() generated
	// ("std::vector<std::pair<cv::dnn::Backend, cv::dnn::Target>>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_delete(std::vector<std::pair<cv::dnn::Backend, cv::dnn::Target>>* instance) {
			delete instance;
	}

	// std::vector<std::pair<cv::dnn::Backend, cv::dnn::Target>>::len() generated
	// ("std::vector<std::pair<cv::dnn::Backend, cv::dnn::Target>>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_len_const(const std::vector<std::pair<cv::dnn::Backend, cv::dnn::Target>>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<std::pair<cv::dnn::Backend, cv::dnn::Target>>::isEmpty() generated
	// ("std::vector<std::pair<cv::dnn::Backend, cv::dnn::Target>>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_isEmpty_const(const std::vector<std::pair<cv::dnn::Backend, cv::dnn::Target>>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<std::pair<cv::dnn::Backend, cv::dnn::Target>>::capacity() generated
	// ("std::vector<std::pair<cv::dnn::Backend, cv::dnn::Target>>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_capacity_const(const std::vector<std::pair<cv::dnn::Backend, cv::dnn::Target>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<std::pair<cv::dnn::Backend, cv::dnn::Target>>::shrinkToFit() generated
	// ("std::vector<std::pair<cv::dnn::Backend, cv::dnn::Target>>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_shrinkToFit(std::vector<std::pair<cv::dnn::Backend, cv::dnn::Target>>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<std::pair<cv::dnn::Backend, cv::dnn::Target>>::reserve(Primitive) generated
	// ("std::vector<std::pair<cv::dnn::Backend, cv::dnn::Target>>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_reserve_size_t(std::vector<std::pair<cv::dnn::Backend, cv::dnn::Target>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<std::pair<cv::dnn::Backend, cv::dnn::Target>>::remove(Primitive) generated
	// ("std::vector<std::pair<cv::dnn::Backend, cv::dnn::Target>>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_remove_size_t(std::vector<std::pair<cv::dnn::Backend, cv::dnn::Target>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<std::pair<cv::dnn::Backend, cv::dnn::Target>>::swap(Primitive, Primitive) generated
	// ("std::vector<std::pair<cv::dnn::Backend, cv::dnn::Target>>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_swap_size_t_size_t(std::vector<std::pair<cv::dnn::Backend, cv::dnn::Target>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<std::pair<cv::dnn::Backend, cv::dnn::Target>>::clear() generated
	// ("std::vector<std::pair<cv::dnn::Backend, cv::dnn::Target>>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_clear(std::vector<std::pair<cv::dnn::Backend, cv::dnn::Target>>* instance) {
			instance->clear();
	}

	// std::vector<std::pair<cv::dnn::Backend, cv::dnn::Target>>::push(CppPassByVoidPtr) generated
	// ("std::vector<std::pair<cv::dnn::Backend, cv::dnn::Target>>::push", vec![(pred!(mut, ["val"], ["const std::pair<cv::dnn::Backend, cv::dnn::Target>"]), _)]),
	void std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_push_const_pairLcv_dnn_Backend__cv_dnn_TargetG(std::vector<std::pair<cv::dnn::Backend, cv::dnn::Target>>* instance, const std::pair<cv::dnn::Backend, cv::dnn::Target>* val) {
			instance->push_back(*val);
	}

	// std::vector<std::pair<cv::dnn::Backend, cv::dnn::Target>>::insert(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::pair<cv::dnn::Backend, cv::dnn::Target>>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::pair<cv::dnn::Backend, cv::dnn::Target>"]), _)]),
	void std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_insert_size_t_const_pairLcv_dnn_Backend__cv_dnn_TargetG(std::vector<std::pair<cv::dnn::Backend, cv::dnn::Target>>* instance, size_t index, const std::pair<cv::dnn::Backend, cv::dnn::Target>* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<std::pair<cv::dnn::Backend, cv::dnn::Target>>::get(Primitive) generated
	// ("std::vector<std::pair<cv::dnn::Backend, cv::dnn::Target>>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_get_const_size_t(const std::vector<std::pair<cv::dnn::Backend, cv::dnn::Target>>* instance, size_t index, std::pair<cv::dnn::Backend, cv::dnn::Target>** ocvrs_return) {
			std::pair<cv::dnn::Backend, cv::dnn::Target> ret = (*instance)[index];
			*ocvrs_return = new std::pair<cv::dnn::Backend, cv::dnn::Target>(ret);
	}

	// std::vector<std::pair<cv::dnn::Backend, cv::dnn::Target>>::set(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::pair<cv::dnn::Backend, cv::dnn::Target>>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::pair<cv::dnn::Backend, cv::dnn::Target>"]), _)]),
	void std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_set_size_t_const_pairLcv_dnn_Backend__cv_dnn_TargetG(std::vector<std::pair<cv::dnn::Backend, cv::dnn::Target>>* instance, size_t index, const std::pair<cv::dnn::Backend, cv::dnn::Target>* val) {
			(*instance)[index] = *val;
	}

}


