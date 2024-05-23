extern "C" {
	// std::vector<std::vector<cv::MatShape>>::new() generated
	// ("std::vector<std::vector<cv::MatShape>>::new", vec![(pred!(const, [], []), _)]),
	std::vector<std::vector<cv::MatShape>>* std_vectorLstd_vectorLcv_MatShapeGG_new_const() {
			std::vector<std::vector<cv::MatShape>>* ret = new std::vector<std::vector<cv::MatShape>>();
			return ret;
	}

	// std::vector<std::vector<cv::MatShape>>::delete() generated
	// ("std::vector<std::vector<cv::MatShape>>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_MatShapeGG_delete(std::vector<std::vector<cv::MatShape>>* instance) {
			delete instance;
	}

	// std::vector<std::vector<cv::MatShape>>::len() generated
	// ("std::vector<std::vector<cv::MatShape>>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_vectorLcv_MatShapeGG_len_const(const std::vector<std::vector<cv::MatShape>>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<std::vector<cv::MatShape>>::isEmpty() generated
	// ("std::vector<std::vector<cv::MatShape>>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLstd_vectorLcv_MatShapeGG_isEmpty_const(const std::vector<std::vector<cv::MatShape>>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<std::vector<cv::MatShape>>::capacity() generated
	// ("std::vector<std::vector<cv::MatShape>>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_vectorLcv_MatShapeGG_capacity_const(const std::vector<std::vector<cv::MatShape>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<std::vector<cv::MatShape>>::shrinkToFit() generated
	// ("std::vector<std::vector<cv::MatShape>>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_MatShapeGG_shrinkToFit(std::vector<std::vector<cv::MatShape>>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<std::vector<cv::MatShape>>::reserve(Primitive) generated
	// ("std::vector<std::vector<cv::MatShape>>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLcv_MatShapeGG_reserve_size_t(std::vector<std::vector<cv::MatShape>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<std::vector<cv::MatShape>>::remove(Primitive) generated
	// ("std::vector<std::vector<cv::MatShape>>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLcv_MatShapeGG_remove_size_t(std::vector<std::vector<cv::MatShape>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<std::vector<cv::MatShape>>::swap(Primitive, Primitive) generated
	// ("std::vector<std::vector<cv::MatShape>>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLstd_vectorLcv_MatShapeGG_swap_size_t_size_t(std::vector<std::vector<cv::MatShape>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<std::vector<cv::MatShape>>::clear() generated
	// ("std::vector<std::vector<cv::MatShape>>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_vectorLcv_MatShapeGG_clear(std::vector<std::vector<cv::MatShape>>* instance) {
			instance->clear();
	}

	// std::vector<std::vector<cv::MatShape>>::push(CppPassByVoidPtr) generated
	// ("std::vector<std::vector<cv::MatShape>>::push", vec![(pred!(mut, ["val"], ["const std::vector<cv::MatShape>"]), _)]),
	void std_vectorLstd_vectorLcv_MatShapeGG_push_const_vectorLMatShapeG(std::vector<std::vector<cv::MatShape>>* instance, const std::vector<cv::MatShape>* val) {
			instance->push_back(*val);
	}

	// std::vector<std::vector<cv::MatShape>>::insert(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::vector<cv::MatShape>>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::vector<cv::MatShape>"]), _)]),
	void std_vectorLstd_vectorLcv_MatShapeGG_insert_size_t_const_vectorLMatShapeG(std::vector<std::vector<cv::MatShape>>* instance, size_t index, const std::vector<cv::MatShape>* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<std::vector<cv::MatShape>>::get(Primitive) generated
	// ("std::vector<std::vector<cv::MatShape>>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_vectorLcv_MatShapeGG_get_const_size_t(const std::vector<std::vector<cv::MatShape>>* instance, size_t index, std::vector<cv::MatShape>** ocvrs_return) {
			std::vector<cv::MatShape> ret = (*instance)[index];
			*ocvrs_return = new std::vector<cv::MatShape>(ret);
	}

	// std::vector<std::vector<cv::MatShape>>::set(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::vector<cv::MatShape>>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::vector<cv::MatShape>"]), _)]),
	void std_vectorLstd_vectorLcv_MatShapeGG_set_size_t_const_vectorLMatShapeG(std::vector<std::vector<cv::MatShape>>* instance, size_t index, const std::vector<cv::MatShape>* val) {
			(*instance)[index] = *val;
	}

}


