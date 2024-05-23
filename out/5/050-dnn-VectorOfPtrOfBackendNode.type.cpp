extern "C" {
	// std::vector<cv::Ptr<cv::dnn::BackendNode>>::new() generated
	// ("std::vector<cv::Ptr<cv::dnn::BackendNode>>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Ptr<cv::dnn::BackendNode>>* std_vectorLcv_PtrLcv_dnn_BackendNodeGG_new_const() {
			std::vector<cv::Ptr<cv::dnn::BackendNode>>* ret = new std::vector<cv::Ptr<cv::dnn::BackendNode>>();
			return ret;
	}

	// std::vector<cv::Ptr<cv::dnn::BackendNode>>::delete() generated
	// ("std::vector<cv::Ptr<cv::dnn::BackendNode>>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_PtrLcv_dnn_BackendNodeGG_delete(std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance) {
			delete instance;
	}

	// std::vector<cv::Ptr<cv::dnn::BackendNode>>::len() generated
	// ("std::vector<cv::Ptr<cv::dnn::BackendNode>>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_PtrLcv_dnn_BackendNodeGG_len_const(const std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::Ptr<cv::dnn::BackendNode>>::isEmpty() generated
	// ("std::vector<cv::Ptr<cv::dnn::BackendNode>>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_PtrLcv_dnn_BackendNodeGG_isEmpty_const(const std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::Ptr<cv::dnn::BackendNode>>::capacity() generated
	// ("std::vector<cv::Ptr<cv::dnn::BackendNode>>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_PtrLcv_dnn_BackendNodeGG_capacity_const(const std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::Ptr<cv::dnn::BackendNode>>::shrinkToFit() generated
	// ("std::vector<cv::Ptr<cv::dnn::BackendNode>>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_PtrLcv_dnn_BackendNodeGG_shrinkToFit(std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::Ptr<cv::dnn::BackendNode>>::reserve(Primitive) generated
	// ("std::vector<cv::Ptr<cv::dnn::BackendNode>>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_PtrLcv_dnn_BackendNodeGG_reserve_size_t(std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::Ptr<cv::dnn::BackendNode>>::remove(Primitive) generated
	// ("std::vector<cv::Ptr<cv::dnn::BackendNode>>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_PtrLcv_dnn_BackendNodeGG_remove_size_t(std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::Ptr<cv::dnn::BackendNode>>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::Ptr<cv::dnn::BackendNode>>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_PtrLcv_dnn_BackendNodeGG_swap_size_t_size_t(std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::Ptr<cv::dnn::BackendNode>>::clear() generated
	// ("std::vector<cv::Ptr<cv::dnn::BackendNode>>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_PtrLcv_dnn_BackendNodeGG_clear(std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance) {
			instance->clear();
	}

	// std::vector<cv::Ptr<cv::dnn::BackendNode>>::push(CppPassByVoidPtr) generated
	// ("std::vector<cv::Ptr<cv::dnn::BackendNode>>::push", vec![(pred!(mut, ["val"], ["const cv::Ptr<cv::dnn::BackendNode>"]), _)]),
	void std_vectorLcv_PtrLcv_dnn_BackendNodeGG_push_const_PtrLBackendNodeG(std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance, const cv::Ptr<cv::dnn::BackendNode>* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::Ptr<cv::dnn::BackendNode>>::insert(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<cv::Ptr<cv::dnn::BackendNode>>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Ptr<cv::dnn::BackendNode>"]), _)]),
	void std_vectorLcv_PtrLcv_dnn_BackendNodeGG_insert_size_t_const_PtrLBackendNodeG(std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance, size_t index, const cv::Ptr<cv::dnn::BackendNode>* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::Ptr<cv::dnn::BackendNode>>::get(Primitive) generated
	// ("std::vector<cv::Ptr<cv::dnn::BackendNode>>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_PtrLcv_dnn_BackendNodeGG_get_const_size_t(const std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance, size_t index, cv::Ptr<cv::dnn::BackendNode>** ocvrs_return) {
			cv::Ptr<cv::dnn::BackendNode> ret = (*instance)[index];
			*ocvrs_return = new cv::Ptr<cv::dnn::BackendNode>(ret);
	}

	// std::vector<cv::Ptr<cv::dnn::BackendNode>>::set(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<cv::Ptr<cv::dnn::BackendNode>>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Ptr<cv::dnn::BackendNode>"]), _)]),
	void std_vectorLcv_PtrLcv_dnn_BackendNodeGG_set_size_t_const_PtrLBackendNodeG(std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance, size_t index, const cv::Ptr<cv::dnn::BackendNode>* val) {
			(*instance)[index] = *val;
	}

}


