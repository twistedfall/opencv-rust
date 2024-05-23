extern "C" {
	// std::vector<std::vector<cv::dnn::MatShape>>::new() generated
	// ("std::vector<std::vector<cv::dnn::MatShape>>::new", vec![(pred!(const, [], []), _)]),
	std::vector<std::vector<cv::dnn::MatShape>>* std_vectorLstd_vectorLcv_dnn_MatShapeGG_new_const() {
			std::vector<std::vector<cv::dnn::MatShape>>* ret = new std::vector<std::vector<cv::dnn::MatShape>>();
			return ret;
	}

	// std::vector<std::vector<cv::dnn::MatShape>>::delete() generated
	// ("std::vector<std::vector<cv::dnn::MatShape>>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_dnn_MatShapeGG_delete(std::vector<std::vector<cv::dnn::MatShape>>* instance) {
			delete instance;
	}

	// std::vector<std::vector<cv::dnn::MatShape>>::len() generated
	// ("std::vector<std::vector<cv::dnn::MatShape>>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_vectorLcv_dnn_MatShapeGG_len_const(const std::vector<std::vector<cv::dnn::MatShape>>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<std::vector<cv::dnn::MatShape>>::isEmpty() generated
	// ("std::vector<std::vector<cv::dnn::MatShape>>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLstd_vectorLcv_dnn_MatShapeGG_isEmpty_const(const std::vector<std::vector<cv::dnn::MatShape>>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<std::vector<cv::dnn::MatShape>>::capacity() generated
	// ("std::vector<std::vector<cv::dnn::MatShape>>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_vectorLcv_dnn_MatShapeGG_capacity_const(const std::vector<std::vector<cv::dnn::MatShape>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<std::vector<cv::dnn::MatShape>>::shrinkToFit() generated
	// ("std::vector<std::vector<cv::dnn::MatShape>>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_dnn_MatShapeGG_shrinkToFit(std::vector<std::vector<cv::dnn::MatShape>>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<std::vector<cv::dnn::MatShape>>::reserve(Primitive) generated
	// ("std::vector<std::vector<cv::dnn::MatShape>>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLcv_dnn_MatShapeGG_reserve_size_t(std::vector<std::vector<cv::dnn::MatShape>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<std::vector<cv::dnn::MatShape>>::remove(Primitive) generated
	// ("std::vector<std::vector<cv::dnn::MatShape>>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLcv_dnn_MatShapeGG_remove_size_t(std::vector<std::vector<cv::dnn::MatShape>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<std::vector<cv::dnn::MatShape>>::swap(Primitive, Primitive) generated
	// ("std::vector<std::vector<cv::dnn::MatShape>>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLstd_vectorLcv_dnn_MatShapeGG_swap_size_t_size_t(std::vector<std::vector<cv::dnn::MatShape>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<std::vector<cv::dnn::MatShape>>::clear() generated
	// ("std::vector<std::vector<cv::dnn::MatShape>>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_dnn_MatShapeGG_clear(std::vector<std::vector<cv::dnn::MatShape>>* instance) {
			instance->clear();
	}

	// std::vector<std::vector<cv::dnn::MatShape>>::push(CppPassByVoidPtr) generated
	// ("std::vector<std::vector<cv::dnn::MatShape>>::push", vec![(pred!(mut, ["val"], ["const std::vector<cv::dnn::MatShape>"]), _)]),
	void std_vectorLstd_vectorLcv_dnn_MatShapeGG_push_const_vectorLMatShapeG(std::vector<std::vector<cv::dnn::MatShape>>* instance, const std::vector<cv::dnn::MatShape>* val) {
			instance->push_back(*val);
	}

	// std::vector<std::vector<cv::dnn::MatShape>>::insert(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::vector<cv::dnn::MatShape>>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::vector<cv::dnn::MatShape>"]), _)]),
	void std_vectorLstd_vectorLcv_dnn_MatShapeGG_insert_size_t_const_vectorLMatShapeG(std::vector<std::vector<cv::dnn::MatShape>>* instance, size_t index, const std::vector<cv::dnn::MatShape>* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<std::vector<cv::dnn::MatShape>>::get(Primitive) generated
	// ("std::vector<std::vector<cv::dnn::MatShape>>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLcv_dnn_MatShapeGG_get_const_size_t(const std::vector<std::vector<cv::dnn::MatShape>>* instance, size_t index, std::vector<cv::dnn::MatShape>** ocvrs_return) {
			std::vector<cv::dnn::MatShape> ret = (*instance)[index];
			*ocvrs_return = new std::vector<cv::dnn::MatShape>(ret);
	}

	// std::vector<std::vector<cv::dnn::MatShape>>::set(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::vector<cv::dnn::MatShape>>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::vector<cv::dnn::MatShape>"]), _)]),
	void std_vectorLstd_vectorLcv_dnn_MatShapeGG_set_size_t_const_vectorLMatShapeG(std::vector<std::vector<cv::dnn::MatShape>>* instance, size_t index, const std::vector<cv::dnn::MatShape>* val) {
			(*instance)[index] = *val;
	}

}


