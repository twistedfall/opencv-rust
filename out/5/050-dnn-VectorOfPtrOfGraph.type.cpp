extern "C" {
	// std::vector<cv::Ptr<cv::dnn::Graph>>::new() generated
	// ("std::vector<cv::Ptr<cv::dnn::Graph>>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Ptr<cv::dnn::Graph>>* std_vectorLcv_PtrLcv_dnn_GraphGG_new_const() {
			std::vector<cv::Ptr<cv::dnn::Graph>>* ret = new std::vector<cv::Ptr<cv::dnn::Graph>>();
			return ret;
	}

	// std::vector<cv::Ptr<cv::dnn::Graph>>::delete() generated
	// ("std::vector<cv::Ptr<cv::dnn::Graph>>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_PtrLcv_dnn_GraphGG_delete(std::vector<cv::Ptr<cv::dnn::Graph>>* instance) {
			delete instance;
	}

	// std::vector<cv::Ptr<cv::dnn::Graph>>::len() generated
	// ("std::vector<cv::Ptr<cv::dnn::Graph>>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_PtrLcv_dnn_GraphGG_len_const(const std::vector<cv::Ptr<cv::dnn::Graph>>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::Ptr<cv::dnn::Graph>>::isEmpty() generated
	// ("std::vector<cv::Ptr<cv::dnn::Graph>>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_PtrLcv_dnn_GraphGG_isEmpty_const(const std::vector<cv::Ptr<cv::dnn::Graph>>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::Ptr<cv::dnn::Graph>>::capacity() generated
	// ("std::vector<cv::Ptr<cv::dnn::Graph>>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_PtrLcv_dnn_GraphGG_capacity_const(const std::vector<cv::Ptr<cv::dnn::Graph>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::Ptr<cv::dnn::Graph>>::shrinkToFit() generated
	// ("std::vector<cv::Ptr<cv::dnn::Graph>>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_PtrLcv_dnn_GraphGG_shrinkToFit(std::vector<cv::Ptr<cv::dnn::Graph>>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::Ptr<cv::dnn::Graph>>::reserve(Primitive) generated
	// ("std::vector<cv::Ptr<cv::dnn::Graph>>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_PtrLcv_dnn_GraphGG_reserve_size_t(std::vector<cv::Ptr<cv::dnn::Graph>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::Ptr<cv::dnn::Graph>>::remove(Primitive) generated
	// ("std::vector<cv::Ptr<cv::dnn::Graph>>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_PtrLcv_dnn_GraphGG_remove_size_t(std::vector<cv::Ptr<cv::dnn::Graph>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::Ptr<cv::dnn::Graph>>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::Ptr<cv::dnn::Graph>>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_PtrLcv_dnn_GraphGG_swap_size_t_size_t(std::vector<cv::Ptr<cv::dnn::Graph>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::Ptr<cv::dnn::Graph>>::clear() generated
	// ("std::vector<cv::Ptr<cv::dnn::Graph>>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_PtrLcv_dnn_GraphGG_clear(std::vector<cv::Ptr<cv::dnn::Graph>>* instance) {
			instance->clear();
	}

	// std::vector<cv::Ptr<cv::dnn::Graph>>::push(CppPassByVoidPtr) generated
	// ("std::vector<cv::Ptr<cv::dnn::Graph>>::push", vec![(pred!(mut, ["val"], ["const cv::Ptr<cv::dnn::Graph>"]), _)]),
	void std_vectorLcv_PtrLcv_dnn_GraphGG_push_const_PtrLGraphG(std::vector<cv::Ptr<cv::dnn::Graph>>* instance, const cv::Ptr<cv::dnn::Graph>* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::Ptr<cv::dnn::Graph>>::insert(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<cv::Ptr<cv::dnn::Graph>>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Ptr<cv::dnn::Graph>"]), _)]),
	void std_vectorLcv_PtrLcv_dnn_GraphGG_insert_size_t_const_PtrLGraphG(std::vector<cv::Ptr<cv::dnn::Graph>>* instance, size_t index, const cv::Ptr<cv::dnn::Graph>* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::Ptr<cv::dnn::Graph>>::get(Primitive) generated
	// ("std::vector<cv::Ptr<cv::dnn::Graph>>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_PtrLcv_dnn_GraphGG_get_const_size_t(const std::vector<cv::Ptr<cv::dnn::Graph>>* instance, size_t index, cv::Ptr<cv::dnn::Graph>** ocvrs_return) {
			cv::Ptr<cv::dnn::Graph> ret = (*instance)[index];
			*ocvrs_return = new cv::Ptr<cv::dnn::Graph>(ret);
	}

	// std::vector<cv::Ptr<cv::dnn::Graph>>::set(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<cv::Ptr<cv::dnn::Graph>>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Ptr<cv::dnn::Graph>"]), _)]),
	void std_vectorLcv_PtrLcv_dnn_GraphGG_set_size_t_const_PtrLGraphG(std::vector<cv::Ptr<cv::dnn::Graph>>* instance, size_t index, const cv::Ptr<cv::dnn::Graph>* val) {
			(*instance)[index] = *val;
	}

}


