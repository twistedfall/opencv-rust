extern "C" {
	// std::vector<cv::Ptr<cv::dnn::Layer>>::new() generated
	// ("std::vector<cv::Ptr<cv::dnn::Layer>>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Ptr<cv::dnn::Layer>>* std_vectorLcv_PtrLcv_dnn_LayerGG_new_const() {
			std::vector<cv::Ptr<cv::dnn::Layer>>* ret = new std::vector<cv::Ptr<cv::dnn::Layer>>();
			return ret;
	}

	// std::vector<cv::Ptr<cv::dnn::Layer>>::delete() generated
	// ("std::vector<cv::Ptr<cv::dnn::Layer>>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_PtrLcv_dnn_LayerGG_delete(std::vector<cv::Ptr<cv::dnn::Layer>>* instance) {
			delete instance;
	}

	// std::vector<cv::Ptr<cv::dnn::Layer>>::len() generated
	// ("std::vector<cv::Ptr<cv::dnn::Layer>>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_PtrLcv_dnn_LayerGG_len_const(const std::vector<cv::Ptr<cv::dnn::Layer>>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::Ptr<cv::dnn::Layer>>::isEmpty() generated
	// ("std::vector<cv::Ptr<cv::dnn::Layer>>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_PtrLcv_dnn_LayerGG_isEmpty_const(const std::vector<cv::Ptr<cv::dnn::Layer>>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::Ptr<cv::dnn::Layer>>::capacity() generated
	// ("std::vector<cv::Ptr<cv::dnn::Layer>>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_PtrLcv_dnn_LayerGG_capacity_const(const std::vector<cv::Ptr<cv::dnn::Layer>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::Ptr<cv::dnn::Layer>>::shrinkToFit() generated
	// ("std::vector<cv::Ptr<cv::dnn::Layer>>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_PtrLcv_dnn_LayerGG_shrinkToFit(std::vector<cv::Ptr<cv::dnn::Layer>>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::Ptr<cv::dnn::Layer>>::reserve(Primitive) generated
	// ("std::vector<cv::Ptr<cv::dnn::Layer>>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_PtrLcv_dnn_LayerGG_reserve_size_t(std::vector<cv::Ptr<cv::dnn::Layer>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::Ptr<cv::dnn::Layer>>::remove(Primitive) generated
	// ("std::vector<cv::Ptr<cv::dnn::Layer>>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_PtrLcv_dnn_LayerGG_remove_size_t(std::vector<cv::Ptr<cv::dnn::Layer>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::Ptr<cv::dnn::Layer>>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::Ptr<cv::dnn::Layer>>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_PtrLcv_dnn_LayerGG_swap_size_t_size_t(std::vector<cv::Ptr<cv::dnn::Layer>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::Ptr<cv::dnn::Layer>>::clear() generated
	// ("std::vector<cv::Ptr<cv::dnn::Layer>>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_PtrLcv_dnn_LayerGG_clear(std::vector<cv::Ptr<cv::dnn::Layer>>* instance) {
			instance->clear();
	}

	// std::vector<cv::Ptr<cv::dnn::Layer>>::push(CppPassByVoidPtr) generated
	// ("std::vector<cv::Ptr<cv::dnn::Layer>>::push", vec![(pred!(mut, ["val"], ["const cv::Ptr<cv::dnn::Layer>"]), _)]),
	void std_vectorLcv_PtrLcv_dnn_LayerGG_push_const_PtrLLayerG(std::vector<cv::Ptr<cv::dnn::Layer>>* instance, const cv::Ptr<cv::dnn::Layer>* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::Ptr<cv::dnn::Layer>>::insert(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<cv::Ptr<cv::dnn::Layer>>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Ptr<cv::dnn::Layer>"]), _)]),
	void std_vectorLcv_PtrLcv_dnn_LayerGG_insert_size_t_const_PtrLLayerG(std::vector<cv::Ptr<cv::dnn::Layer>>* instance, size_t index, const cv::Ptr<cv::dnn::Layer>* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::Ptr<cv::dnn::Layer>>::get(Primitive) generated
	// ("std::vector<cv::Ptr<cv::dnn::Layer>>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_PtrLcv_dnn_LayerGG_get_const_size_t(const std::vector<cv::Ptr<cv::dnn::Layer>>* instance, size_t index, cv::Ptr<cv::dnn::Layer>** ocvrs_return) {
			cv::Ptr<cv::dnn::Layer> ret = (*instance)[index];
			*ocvrs_return = new cv::Ptr<cv::dnn::Layer>(ret);
	}

	// std::vector<cv::Ptr<cv::dnn::Layer>>::set(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<cv::Ptr<cv::dnn::Layer>>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Ptr<cv::dnn::Layer>"]), _)]),
	void std_vectorLcv_PtrLcv_dnn_LayerGG_set_size_t_const_PtrLLayerG(std::vector<cv::Ptr<cv::dnn::Layer>>* instance, size_t index, const cv::Ptr<cv::dnn::Layer>* val) {
			(*instance)[index] = *val;
	}

}


