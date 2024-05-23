extern "C" {
	// std::vector<cv::Ptr<cv::dnn::BackendWrapper>>::new() generated
	// ("std::vector<cv::Ptr<cv::dnn::BackendWrapper>>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_new_const() {
			std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* ret = new std::vector<cv::Ptr<cv::dnn::BackendWrapper>>();
			return ret;
	}

	// std::vector<cv::Ptr<cv::dnn::BackendWrapper>>::delete() generated
	// ("std::vector<cv::Ptr<cv::dnn::BackendWrapper>>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_delete(std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance) {
			delete instance;
	}

	// std::vector<cv::Ptr<cv::dnn::BackendWrapper>>::len() generated
	// ("std::vector<cv::Ptr<cv::dnn::BackendWrapper>>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_len_const(const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::Ptr<cv::dnn::BackendWrapper>>::isEmpty() generated
	// ("std::vector<cv::Ptr<cv::dnn::BackendWrapper>>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_isEmpty_const(const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::Ptr<cv::dnn::BackendWrapper>>::capacity() generated
	// ("std::vector<cv::Ptr<cv::dnn::BackendWrapper>>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_capacity_const(const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::Ptr<cv::dnn::BackendWrapper>>::shrinkToFit() generated
	// ("std::vector<cv::Ptr<cv::dnn::BackendWrapper>>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_shrinkToFit(std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::Ptr<cv::dnn::BackendWrapper>>::reserve(Primitive) generated
	// ("std::vector<cv::Ptr<cv::dnn::BackendWrapper>>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_reserve_size_t(std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::Ptr<cv::dnn::BackendWrapper>>::remove(Primitive) generated
	// ("std::vector<cv::Ptr<cv::dnn::BackendWrapper>>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_remove_size_t(std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::Ptr<cv::dnn::BackendWrapper>>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::Ptr<cv::dnn::BackendWrapper>>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_swap_size_t_size_t(std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::Ptr<cv::dnn::BackendWrapper>>::clear() generated
	// ("std::vector<cv::Ptr<cv::dnn::BackendWrapper>>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_clear(std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance) {
			instance->clear();
	}

	// std::vector<cv::Ptr<cv::dnn::BackendWrapper>>::push(CppPassByVoidPtr) generated
	// ("std::vector<cv::Ptr<cv::dnn::BackendWrapper>>::push", vec![(pred!(mut, ["val"], ["const cv::Ptr<cv::dnn::BackendWrapper>"]), _)]),
	void std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_push_const_PtrLBackendWrapperG(std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance, const cv::Ptr<cv::dnn::BackendWrapper>* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::Ptr<cv::dnn::BackendWrapper>>::insert(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<cv::Ptr<cv::dnn::BackendWrapper>>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Ptr<cv::dnn::BackendWrapper>"]), _)]),
	void std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_insert_size_t_const_PtrLBackendWrapperG(std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance, size_t index, const cv::Ptr<cv::dnn::BackendWrapper>* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::Ptr<cv::dnn::BackendWrapper>>::get(Primitive) generated
	// ("std::vector<cv::Ptr<cv::dnn::BackendWrapper>>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_get_const_size_t(const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance, size_t index, cv::Ptr<cv::dnn::BackendWrapper>** ocvrs_return) {
			cv::Ptr<cv::dnn::BackendWrapper> ret = (*instance)[index];
			*ocvrs_return = new cv::Ptr<cv::dnn::BackendWrapper>(ret);
	}

	// std::vector<cv::Ptr<cv::dnn::BackendWrapper>>::set(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<cv::Ptr<cv::dnn::BackendWrapper>>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Ptr<cv::dnn::BackendWrapper>"]), _)]),
	void std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_set_size_t_const_PtrLBackendWrapperG(std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance, size_t index, const cv::Ptr<cv::dnn::BackendWrapper>* val) {
			(*instance)[index] = *val;
	}

}


